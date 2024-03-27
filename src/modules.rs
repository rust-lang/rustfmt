use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rustc_arena::TypedArena;
use rustc_ast::ast;
use rustc_ast::ptr::P;
use rustc_ast::visit::Visitor;
use rustc_span::symbol::{self, sym, Symbol};
use rustc_span::Span;
use thiserror::Error;

use crate::attr::MetaVisitor;
use crate::config::FileName;
use crate::parse::parser::{
    Directory, DirectoryOwnership, ModError, ModulePathSuccess, Parser, ParserError,
};
use crate::parse::session::ParseSess;
use crate::utils::{contains_skip, mk_sp};

mod visitor;

type FileModMap<'ast> = BTreeMap<FileName, Module<'ast>>;

/// Represents module with its inner attributes.
#[derive(Debug, Clone)]
pub(crate) struct Module<'a> {
    pub(crate) items: &'a [rustc_ast::ptr::P<ast::Item>],
    inner_attr: ast::AttrVec,
    pub(crate) span: Span,
}

impl<'a> Module<'a> {
    pub(crate) fn new(
        mod_span: Span,
        mod_items: &'a [rustc_ast::ptr::P<ast::Item>],
        mod_attrs: &[ast::Attribute],
    ) -> Self {
        let inner_attr = mod_attrs
            .iter()
            .filter(|attr| attr.style == ast::AttrStyle::Inner)
            .cloned()
            .collect();
        Module {
            items: mod_items,
            inner_attr,
            span: mod_span,
        }
    }

    pub(crate) fn from_item(item: &'a ast::Item) -> Module<'a> {
        let items = match &item.kind {
            ast::ItemKind::Mod(_, ast::ModKind::Loaded(items, ..)) => &**items,
            _ => &[],
        };
        Module::new(item.span, items, &item.attrs)
    }

    pub(crate) fn attrs(&self) -> &[ast::Attribute] {
        &self.inner_attr
    }
}

/// Maps each module to the corresponding file.
pub(crate) struct ModResolver<'ast, 'sess> {
    item_arena: &'ast TypedArena<P<ast::Item>>,
    parse_sess: &'sess ParseSess,
    directory: Directory,
    file_map: FileModMap<'ast>,
    recursive: bool,
}

/// Represents errors while trying to resolve modules.
#[derive(Debug, Error)]
#[error("failed to resolve mod `{module}`: {kind}")]
pub struct ModuleResolutionError {
    pub(crate) module: String,
    pub(crate) kind: ModuleResolutionErrorKind,
}

/// Defines variants similar to those of [rustc_expand::module::ModError]
#[derive(Debug, Error)]
pub(crate) enum ModuleResolutionErrorKind {
    /// Find a file that cannot be parsed.
    #[error("cannot parse {file}")]
    ParseError { file: PathBuf },
    /// File cannot be found.
    #[error("{file} does not exist")]
    NotFound { file: PathBuf },
    /// File a.rs and a/mod.rs both exist
    #[error("file for module found at both {default_path:?} and {secondary_path:?}")]
    MultipleCandidates {
        default_path: PathBuf,
        secondary_path: PathBuf,
    },
}

#[derive(Clone)]
enum SubModKind<'ast> {
    /// `mod foo;`
    External(PathBuf, DirectoryOwnership, Module<'ast>),
    /// `mod foo;` with multiple sources.
    MultiExternal(Vec<(PathBuf, DirectoryOwnership, Module<'ast>)>),
}

impl<'ast, 'sess> ModResolver<'ast, 'sess> {
    /// Creates a new `ModResolver`.
    pub(crate) fn new(
        item_arena: &'ast TypedArena<P<ast::Item>>,
        parse_sess: &'sess ParseSess,
        directory_ownership: DirectoryOwnership,
        recursive: bool,
    ) -> Self {
        ModResolver {
            item_arena,
            directory: Directory {
                path: PathBuf::new(),
                ownership: directory_ownership,
            },
            file_map: BTreeMap::new(),
            parse_sess,
            recursive,
        }
    }

    /// Creates a map that maps a file name to the module in AST.
    pub(crate) fn visit_crate(
        mut self,
        krate: &'ast ast::Crate,
    ) -> Result<FileModMap<'ast>, ModuleResolutionError> {
        let root_filename = self.parse_sess.span_to_filename(krate.spans.inner_span);
        self.directory.path = match root_filename {
            FileName::Real(ref p) => p.parent().unwrap_or(Path::new("")).to_path_buf(),
            _ => PathBuf::new(),
        };

        // Skip visiting sub modules when the input is from stdin.
        if self.recursive {
            self.visit_items(&krate.items)?;
        }

        let snippet_provider = self.parse_sess.snippet_provider(krate.spans.inner_span);

        self.file_map.insert(
            root_filename,
            Module::new(
                mk_sp(snippet_provider.start_pos(), snippet_provider.end_pos()),
                &krate.items,
                &krate.attrs,
            ),
        );
        Ok(self.file_map)
    }

    /// Visit `cfg_if` macro and look for module declarations.
    fn visit_cfg_if(&mut self, item: &ast::Item) -> Result<(), ModuleResolutionError> {
        let mut visitor = visitor::CfgIfVisitor::new(self.parse_sess);
        visitor.visit_item(item);
        let items = self.item_arena.alloc_from_iter(visitor.items);
        self.visit_items(&*items)?;
        Ok(())
    }

    fn visit_items(
        &mut self,
        items: &'ast [rustc_ast::ptr::P<ast::Item>],
    ) -> Result<(), ModuleResolutionError> {
        for item in items {
            if is_cfg_if(item) {
                self.visit_cfg_if(item)?;
            } else if let ast::ItemKind::Mod(_, mod_kind) = &item.kind {
                self.visit_mod(&item, mod_kind)?;
            }
        }
        Ok(())
    }

    fn visit_mod(
        &mut self,
        item: &'ast ast::Item,
        mod_kind: &'ast ast::ModKind,
    ) -> Result<(), ModuleResolutionError> {
        if contains_skip(&item.attrs) {
            return Ok(());
        }
        match mod_kind {
            ast::ModKind::Loaded(items, ast::Inline::Yes, _) => {
                // An internal module (`mod foo { /* ... */ }`);
                let directory = self.inline_mod_directory(item.ident, &item.attrs);
                self.with_directory(directory, |this| this.visit_items(items))?;
            }
            _ => {
                // mod foo;
                // Look for an extern file.
                let Some(kind) = self.find_external_module(item)? else {
                    return Ok(());
                };
                self.insert_sub_mod(kind.clone())?;
                self.visit_sub_mod_inner(kind)?;
            }
        }
        Ok(())
    }

    fn insert_sub_mod(
        &mut self,
        sub_mod_kind: SubModKind<'ast>,
    ) -> Result<(), ModuleResolutionError> {
        match sub_mod_kind {
            SubModKind::External(mod_path, _, sub_mod) => {
                self.file_map
                    .entry(FileName::Real(mod_path))
                    .or_insert(sub_mod);
            }
            SubModKind::MultiExternal(mods) => {
                for (mod_path, _, sub_mod) in mods {
                    self.file_map
                        .entry(FileName::Real(mod_path))
                        .or_insert(sub_mod);
                }
            }
        }
        Ok(())
    }

    fn visit_sub_mod_inner(
        &mut self,
        sub_mod_kind: SubModKind<'ast>,
    ) -> Result<(), ModuleResolutionError> {
        match sub_mod_kind {
            SubModKind::External(mod_path, directory_ownership, sub_mod) => {
                let directory = Directory {
                    path: mod_path.parent().unwrap().to_path_buf(),
                    ownership: directory_ownership,
                };
                self.with_directory(directory, |this| this.visit_items(&sub_mod.items))?;
            }
            SubModKind::MultiExternal(mods) => {
                for (mod_path, directory_ownership, sub_mod) in mods {
                    let directory = Directory {
                        path: mod_path.parent().unwrap().to_path_buf(),
                        ownership: directory_ownership,
                    };
                    self.with_directory(directory, |this| this.visit_items(&sub_mod.items))?;
                }
            }
        }
        Ok(())
    }

    /// Find a file path in the filesystem which corresponds to the given module.
    fn find_external_module(
        &self,
        item: &'ast ast::Item,
    ) -> Result<Option<SubModKind<'ast>>, ModuleResolutionError> {
        let mod_name = item.ident;
        let relative = match self.directory.ownership {
            DirectoryOwnership::Owned { relative } => relative,
            DirectoryOwnership::UnownedViaBlock => None,
        };
        if let Some(path) = Parser::submod_path_from_attr(&item.attrs, &self.directory.path) {
            if self.parse_sess.is_file_parsed(&path) {
                return Ok(None);
            }
            return match Parser::parse_file_as_module(self.parse_sess, &path, item.span) {
                Ok((ref attrs, _, _)) if contains_skip(attrs) => Ok(None),
                Ok((attrs, items, span)) => Ok(Some(SubModKind::External(
                    path,
                    DirectoryOwnership::Owned { relative: None },
                    Module::new(span, self.item_arena.alloc_from_iter(items), &attrs),
                ))),
                Err(ParserError::ParseError) => Err(ModuleResolutionError {
                    module: mod_name.to_string(),
                    kind: ModuleResolutionErrorKind::ParseError { file: path },
                }),
                Err(..) => Err(ModuleResolutionError {
                    module: mod_name.to_string(),
                    kind: ModuleResolutionErrorKind::NotFound { file: path },
                }),
            };
        }

        // Look for nested path, like `#[cfg_attr(feature = "foo", path = "bar.rs")]`.
        let mut mods_outside_ast = self.find_mods_outside_of_ast(item);

        match self
            .parse_sess
            .default_submod_path(mod_name, relative, &self.directory.path)
        {
            Ok(ModulePathSuccess {
                file_path,
                dir_ownership,
                ..
            }) => {
                let outside_mods_empty = mods_outside_ast.is_empty();
                let should_insert = !mods_outside_ast
                    .iter()
                    .any(|(outside_path, _, _)| outside_path == &file_path);
                if self.parse_sess.is_file_parsed(&file_path) {
                    if outside_mods_empty {
                        return Ok(None);
                    } else {
                        if should_insert {
                            mods_outside_ast.push((
                                file_path,
                                dir_ownership,
                                Module::from_item(item),
                            ));
                        }
                        return Ok(Some(SubModKind::MultiExternal(mods_outside_ast)));
                    }
                }
                match Parser::parse_file_as_module(self.parse_sess, &file_path, item.span) {
                    Ok((ref attrs, _, _)) if contains_skip(attrs) => Ok(None),
                    Ok((attrs, items, span)) if outside_mods_empty => {
                        Ok(Some(SubModKind::External(
                            file_path,
                            dir_ownership,
                            Module::new(span, self.item_arena.alloc_from_iter(items), &attrs),
                        )))
                    }
                    Ok((attrs, items, span)) => {
                        mods_outside_ast.push((
                            file_path.clone(),
                            dir_ownership,
                            Module::new(span, self.item_arena.alloc_from_iter(items), &attrs),
                        ));
                        if should_insert {
                            mods_outside_ast.push((
                                file_path,
                                dir_ownership,
                                Module::from_item(item),
                            ));
                        }
                        Ok(Some(SubModKind::MultiExternal(mods_outside_ast)))
                    }
                    Err(ParserError::ParseError) => Err(ModuleResolutionError {
                        module: mod_name.to_string(),
                        kind: ModuleResolutionErrorKind::ParseError { file: file_path },
                    }),
                    Err(..) if outside_mods_empty => Err(ModuleResolutionError {
                        module: mod_name.to_string(),
                        kind: ModuleResolutionErrorKind::NotFound { file: file_path },
                    }),
                    Err(..) => {
                        if should_insert {
                            mods_outside_ast.push((
                                file_path,
                                dir_ownership,
                                Module::from_item(item),
                            ));
                        }
                        Ok(Some(SubModKind::MultiExternal(mods_outside_ast)))
                    }
                }
            }
            Err(mod_err) if !mods_outside_ast.is_empty() => {
                if let ModError::ParserError(e) = mod_err {
                    e.cancel();
                }
                Ok(Some(SubModKind::MultiExternal(mods_outside_ast)))
            }
            Err(e) => match e {
                ModError::FileNotFound(_, default_path, _secondary_path) => {
                    Err(ModuleResolutionError {
                        module: mod_name.to_string(),
                        kind: ModuleResolutionErrorKind::NotFound { file: default_path },
                    })
                }
                ModError::MultipleCandidates(_, default_path, secondary_path) => {
                    Err(ModuleResolutionError {
                        module: mod_name.to_string(),
                        kind: ModuleResolutionErrorKind::MultipleCandidates {
                            default_path,
                            secondary_path,
                        },
                    })
                }
                ModError::ParserError(_)
                | ModError::CircularInclusion(_)
                | ModError::ModInBlock(_) => Err(ModuleResolutionError {
                    module: mod_name.to_string(),
                    kind: ModuleResolutionErrorKind::ParseError {
                        file: self.directory.path.clone(),
                    },
                }),
            },
        }
    }

    fn inline_mod_directory(&mut self, id: symbol::Ident, attrs: &[ast::Attribute]) -> Directory {
        if let Some(path) = find_path_value(attrs) {
            Directory {
                path: self.directory.path.join(path.as_str()),
                ownership: DirectoryOwnership::Owned { relative: None },
            }
        } else {
            let id = id.as_str();
            // We have to push on the current module name in the case of relative
            // paths in order to ensure that any additional module paths from inline
            // `mod x { ... }` come after the relative extension.
            //
            // For example, a `mod z { ... }` inside `x/y.rs` should set the current
            // directory path to `/x/y/z`, not `/x/z` with a relative offset of `y`.
            let new_path = if let DirectoryOwnership::Owned {
                relative: Some(ident),
            } = self.directory.ownership
            {
                // remove the relative offset
                let relative = self.directory.path.join(ident.as_str());
                let nested = relative.join(id);

                // In the case where there is an x.rs and an ./x directory we want
                // to prevent adding x twice. For example, ./x/x
                if relative.exists() && !nested.exists() {
                    relative
                } else {
                    nested
                }
            } else {
                self.directory.path.join(id)
            };
            Directory {
                path: new_path,
                ownership: self.directory.ownership,
            }
        }
    }

    fn find_mods_outside_of_ast(
        &self,
        item: &'ast ast::Item,
    ) -> Vec<(PathBuf, DirectoryOwnership, Module<'ast>)> {
        // Filter nested path, like `#[cfg_attr(feature = "foo", path = "bar.rs")]`.
        let mut path_visitor = visitor::PathVisitor::default();
        for attr in item.attrs.iter() {
            if let Some(meta) = attr.meta() {
                path_visitor.visit_meta_item(&meta)
            }
        }
        let mut result = vec![];
        for path in path_visitor.paths() {
            let actual_path = self.directory.path.join(path);
            if !actual_path.exists() {
                continue;
            }
            if self.parse_sess.is_file_parsed(&actual_path) {
                // If the specified file is already parsed, then we just use that.
                result.push((
                    actual_path,
                    DirectoryOwnership::Owned { relative: None },
                    Module::from_item(item),
                ));
                continue;
            }
            let (attrs, items, span) =
                match Parser::parse_file_as_module(self.parse_sess, &actual_path, item.span) {
                    Ok((ref attrs, _, _)) if contains_skip(attrs) => continue,
                    Ok(m) => m,
                    Err(..) => continue,
                };

            result.push((
                actual_path,
                DirectoryOwnership::Owned { relative: None },
                Module::new(span, self.item_arena.alloc_from_iter(items), &attrs),
            ))
        }
        result
    }

    fn with_directory<T>(&mut self, directory: Directory, f: impl FnOnce(&mut Self) -> T) -> T {
        let old = std::mem::replace(&mut self.directory, directory);
        let out = f(self);
        self.directory = old;
        out
    }
}

fn path_value(attr: &ast::Attribute) -> Option<Symbol> {
    if attr.has_name(sym::path) {
        attr.value_str()
    } else {
        None
    }
}

// N.B., even when there are multiple `#[path = ...]` attributes, we just need to
// examine the first one, since rustc ignores the second and the subsequent ones
// as unused attributes.
fn find_path_value(attrs: &[ast::Attribute]) -> Option<Symbol> {
    attrs.iter().flat_map(path_value).next()
}

fn is_cfg_if(item: &ast::Item) -> bool {
    match item.kind {
        ast::ItemKind::MacCall(ref mac) => {
            if let Some(first_segment) = mac.path.segments.first() {
                if first_segment.ident.name == Symbol::intern("cfg_if") {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}
