// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Reorder items.
//!
//! `mod`, `extern crate` and `use` declarations are reorderd in alphabetical
//! order. Trait items are reordered in pre-determined order (associated types
//! and constatns comes before methods).

// TODO(#2455): Reorder trait items.

use config::{Config, lists::*};
use syntax::{ast, attr, codemap::Span};

use codemap::LineRangeUtils;
use comment::{combine_strs_with_missing_comments, contains_comment};
use format_code_block;
use imports::{path_to_imported_ident, rewrite_import};
use items::rewrite_mod;
use lists::{itemize_list, write_list, ListFormatting};
use rewrite::{Rewrite, RewriteContext};
use shape::Shape;
use spanned::Spanned;
use utils::mk_sp;
use visitor::{filter_inline_attrs, is_use_item, rewrite_extern_crate, FmtVisitor};

use std::{cmp::Ordering, collections::BTreeMap};

fn compare_path_segments(a: &ast::PathSegment, b: &ast::PathSegment) -> Ordering {
    a.identifier.name.as_str().cmp(&b.identifier.name.as_str())
}

fn compare_paths(a: &ast::Path, b: &ast::Path) -> Ordering {
    for segment in a.segments.iter().zip(b.segments.iter()) {
        let ord = compare_path_segments(segment.0, segment.1);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    a.segments.len().cmp(&b.segments.len())
}

fn compare_use_trees(a: &ast::UseTree, b: &ast::UseTree, nested: bool) -> Ordering {
    use ast::UseTreeKind::*;

    // `use_nested_groups` is not yet supported, remove the `if !nested` when support will be
    // fully added
    if !nested {
        let paths_cmp = compare_paths(&a.prefix, &b.prefix);
        if paths_cmp != Ordering::Equal {
            return paths_cmp;
        }
    }

    match (&a.kind, &b.kind) {
        (&Simple(ident_a), &Simple(ident_b)) => {
            let name_a = &*path_to_imported_ident(&a.prefix).name.as_str();
            let name_b = &*path_to_imported_ident(&b.prefix).name.as_str();
            let name_ordering = if name_a == "self" {
                if name_b == "self" {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else if name_b == "self" {
                Ordering::Greater
            } else {
                name_a.cmp(name_b)
            };
            if name_ordering == Ordering::Equal {
                if ident_a.name.as_str() != name_a {
                    if ident_b.name.as_str() != name_b {
                        ident_a.name.as_str().cmp(&ident_b.name.as_str())
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Less
                }
            } else {
                name_ordering
            }
        }
        (&Glob, &Glob) => Ordering::Equal,
        (&Simple(_), _) | (&Glob, &Nested(_)) => Ordering::Less,
        (&Nested(ref a_items), &Nested(ref b_items)) => {
            let mut a = a_items
                .iter()
                .map(|&(ref tree, _)| tree.clone())
                .collect::<Vec<_>>();
            let mut b = b_items
                .iter()
                .map(|&(ref tree, _)| tree.clone())
                .collect::<Vec<_>>();
            a.sort_by(|a, b| compare_use_trees(a, b, true));
            b.sort_by(|a, b| compare_use_trees(a, b, true));
            for comparison_pair in a.iter().zip(b.iter()) {
                let ord = compare_use_trees(comparison_pair.0, comparison_pair.1, true);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            a.len().cmp(&b.len())
        }
        (&Glob, &Simple(_)) | (&Nested(_), _) => Ordering::Greater,
    }
}

/// Choose the ordering between the given two items.
fn compare_items(a: &ast::Item, b: &ast::Item) -> Ordering {
    match (&a.node, &b.node) {
        (&ast::ItemKind::Mod(..), &ast::ItemKind::Mod(..)) => {
            a.ident.name.as_str().cmp(&b.ident.name.as_str())
        }
        (&ast::ItemKind::Use(ref a_tree), &ast::ItemKind::Use(ref b_tree)) => {
            compare_use_trees(a_tree, b_tree, false)
        }
        (&ast::ItemKind::ExternCrate(ref a_name), &ast::ItemKind::ExternCrate(ref b_name)) => {
            // `extern crate foo as bar;`
            //               ^^^ Comparing this.
            let a_orig_name =
                a_name.map_or_else(|| a.ident.name.as_str(), |symbol| symbol.as_str());
            let b_orig_name =
                b_name.map_or_else(|| b.ident.name.as_str(), |symbol| symbol.as_str());
            let result = a_orig_name.cmp(&b_orig_name);
            if result != Ordering::Equal {
                return result;
            }

            // `extern crate foo as bar;`
            //                      ^^^ Comparing this.
            match (a_name, b_name) {
                (Some(..), None) => Ordering::Greater,
                (None, Some(..)) => Ordering::Less,
                (None, None) => Ordering::Equal,
                (Some(..), Some(..)) => a.ident.name.as_str().cmp(&b.ident.name.as_str()),
            }
        }
        _ => unreachable!(),
    }
}

/// Collect names in `use` declarations so that we can merge them which come
/// from the same module.
#[derive(Debug)]
struct UseNameTree {
    subtree: BTreeMap<String, Box<Option<UseNameTree>>>,
}

impl UseNameTree {
    pub fn new() -> UseNameTree {
        UseNameTree {
            subtree: BTreeMap::new(),
        }
    }

    /// Add names that appear in the given `UseTree`.
    pub fn add_use_tree(&mut self, use_tree: &ast::UseTree) {
        // 1 = skip root.
        self.add_use_tree_inner(use_tree, 1)
    }

    fn add_use_tree_inner(&mut self, use_tree: &ast::UseTree, depth: usize) {
        if use_tree.prefix.segments.len() == depth {
            match use_tree.kind {
                ast::UseTreeKind::Glob => {
                    self.subtree.insert("*".to_owned(), box None);
                }
                ast::UseTreeKind::Simple(_) => {
                    self.subtree.insert("self".to_owned(), box None);
                }
                ast::UseTreeKind::Nested(ref nested_use_trees) => {
                    for (nested_use_tree, _) in nested_use_trees {
                        self.add_use_tree_inner(nested_use_tree, 0);
                    }
                }
            };
            return;
        }
        let identifier = use_tree.prefix.segments[depth].identifier.name.to_string();
        if let Some(box Some(ref mut map)) = self.subtree.get_mut(&identifier) {
            map.add_use_tree_inner(use_tree, depth + 1);
            return;
        }
        let mut map = UseNameTree::new();
        map.add_use_tree_inner(use_tree, depth + 1);
        self.subtree.insert(identifier, box Some(map));
    }

    fn to_string_simple_inner(
        &self,
        shape: Shape,
        context: &RewriteContext,
        mut result: &mut String,
    ) {
        for (seg, map) in &self.subtree {
            result.push_str(seg);
            if let box Some(ref map) = map {
                result.push_str("::{");
                let nested_shape = shape.block_indent(context.config.tab_spaces());
                result.push_str(&nested_shape.indent.to_string_with_newline(context.config));
                map.to_string_simple_inner(nested_shape, context, &mut result);
                result.push_str(&shape.indent.to_string_with_newline(context.config));
                result.push('}');
            }
            result.push_str(", ");
        }
    }

    /// Converts this `UseNameTree` to merged `use` declarations. The produced
    /// string is syntactically correct, but is not properly formatted.
    // FIXME: Currently we call `format_code_block` after creating a string with
    // this method. Instead, it would be nice if this method can format the
    // string by itself.
    pub fn to_string_simple(&self, shape: Shape, context: &RewriteContext) -> String {
        let mut result = String::with_capacity(256);
        for (seg, map) in &self.subtree {
            result.push_str("use ");
            let nested_shape = shape.block_indent(context.config.tab_spaces());
            result.push_str(seg);
            result.push_str("::{");
            result.push_str(&nested_shape.indent.to_string_with_newline(context.config));
            if let box Some(ref map) = map {
                map.to_string_simple_inner(nested_shape, context, &mut result);
            }
            result.push_str(&shape.indent.to_string_with_newline(context.config));
            result.push_str("};\n");
        }
        result
    }
}

/// Format `use` declarations. If two or more `use` declarations are from the same module,
/// those will be merged into a single declaration.
///
/// Before calling this function, make sure that there are no comments
/// among/within any `use` declarations.
fn rewrite_use_items_with_merge(
    context: &RewriteContext,
    use_items: Vec<&ast::UseTree>,
    shape: Shape,
) -> Option<String> {
    let mut use_name_tree = UseNameTree::new();
    for use_item in use_items {
        use_name_tree.add_use_tree(use_item);
    }
    let snippet = use_name_tree.to_string_simple(shape, context);
    let mut config = context.config.clone();
    config.set().merge_imports(false);
    format_code_block(&snippet, &config)
}

/// Rewrite a list of items with reordering. Every item in `items` must have
/// the same `ast::ItemKind`.
// TODO (some day) remove unused imports, expand globs, compress many single
// imports into a list import.
fn rewrite_reorderable_items(
    context: &RewriteContext,
    reorderable_items: &[&ast::Item],
    shape: Shape,
    span: Span,
) -> Option<String> {
    // Try to merge `use` declarations from the same module if there are no
    // comments among declarations.
    if context.config.merge_imports() && !contains_comment(context.snippet(span))
        && is_use_item(reorderable_items[0])
    {
        fn get_use_tree<'a>(item: &&'a ast::Item) -> &'a ast::UseTree {
            match item.node {
                ast::ItemKind::Use(ref use_tree) => use_tree,
                _ => unreachable!(),
            }
        }

        return rewrite_use_items_with_merge(
            context,
            reorderable_items
                .iter()
                .map(get_use_tree)
                .collect::<Vec<_>>(),
            shape,
        );
    }

    let items = itemize_list(
        context.snippet_provider,
        reorderable_items.iter(),
        "",
        ";",
        |item| item.span().lo(),
        |item| item.span().hi(),
        |item| {
            let attrs = filter_inline_attrs(&item.attrs, item.span());
            let attrs_str = attrs.rewrite(context, shape)?;

            let missed_span = if attrs.is_empty() {
                mk_sp(item.span.lo(), item.span.lo())
            } else {
                mk_sp(attrs.last().unwrap().span.hi(), item.span.lo())
            };

            let item_str = match item.node {
                ast::ItemKind::Use(ref tree) => {
                    rewrite_import(context, &item.vis, tree, &item.attrs, shape)?
                }
                ast::ItemKind::ExternCrate(..) => rewrite_extern_crate(context, item)?,
                ast::ItemKind::Mod(..) => rewrite_mod(item),
                _ => return None,
            };

            combine_strs_with_missing_comments(
                context,
                &attrs_str,
                &item_str,
                missed_span,
                shape,
                false,
            )
        },
        span.lo(),
        span.hi(),
        false,
    );
    let mut item_pair_vec: Vec<_> = items.zip(reorderable_items.iter()).collect();
    item_pair_vec.sort_by(|a, b| compare_items(a.1, b.1));
    let item_vec: Vec<_> = item_pair_vec.into_iter().map(|pair| pair.0).collect();

    let fmt = ListFormatting {
        tactic: DefinitiveListTactic::Vertical,
        separator: "",
        trailing_separator: SeparatorTactic::Never,
        separator_place: SeparatorPlace::Back,
        shape,
        ends_with_newline: true,
        preserve_newline: false,
        config: context.config,
    };

    write_list(&item_vec, &fmt)
}

fn contains_macro_use_attr(item: &ast::Item) -> bool {
    attr::contains_name(&filter_inline_attrs(&item.attrs, item.span()), "macro_use")
}

/// A simplified version of `ast::ItemKind`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ReorderableItemKind {
    ExternCrate,
    Mod,
    Use,
    /// An item that cannot be reordered. Either has an unreorderable item kind
    /// or an `macro_use` attribute.
    Other,
}

impl ReorderableItemKind {
    pub fn from(item: &ast::Item) -> Self {
        match item.node {
            _ if contains_macro_use_attr(item) => ReorderableItemKind::Other,
            ast::ItemKind::ExternCrate(..) => ReorderableItemKind::ExternCrate,
            ast::ItemKind::Mod(..) => ReorderableItemKind::Mod,
            ast::ItemKind::Use(..) => ReorderableItemKind::Use,
            _ => ReorderableItemKind::Other,
        }
    }

    pub fn is_same_item_kind(&self, item: &ast::Item) -> bool {
        ReorderableItemKind::from(item) == *self
    }

    pub fn is_reorderable(&self, config: &Config) -> bool {
        match *self {
            ReorderableItemKind::ExternCrate => config.reorder_extern_crates(),
            ReorderableItemKind::Mod => config.reorder_modules(),
            ReorderableItemKind::Use => config.reorder_imports(),
            ReorderableItemKind::Other => false,
        }
    }

    pub fn in_group(&self, config: &Config) -> bool {
        match *self {
            ReorderableItemKind::ExternCrate => config.reorder_extern_crates_in_group(),
            ReorderableItemKind::Mod => config.reorder_modules(),
            ReorderableItemKind::Use => config.reorder_imports_in_group(),
            ReorderableItemKind::Other => false,
        }
    }
}

impl<'b, 'a: 'b> FmtVisitor<'a> {
    /// Format items with the same item kind and reorder them. If `in_group` is
    /// `true`, then the items separated by an empty line will not be reordered
    /// together.
    fn walk_reorderable_items(
        &mut self,
        items: &[&ast::Item],
        item_kind: ReorderableItemKind,
        in_group: bool,
    ) -> usize {
        let mut last = self.codemap.lookup_line_range(items[0].span());
        let item_length = items
            .iter()
            .take_while(|ppi| {
                item_kind.is_same_item_kind(&***ppi) && (!in_group || {
                    let current = self.codemap.lookup_line_range(ppi.span());
                    let in_same_group = current.lo < last.hi + 2;
                    last = current;
                    in_same_group
                })
            })
            .count();
        let items = &items[..item_length];

        let at_least_one_in_file_lines = items
            .iter()
            .any(|item| !out_of_file_lines_range!(self, item.span));

        if at_least_one_in_file_lines && !items.is_empty() {
            let lo = items.first().unwrap().span().lo();
            let hi = items.last().unwrap().span().hi();
            let span = mk_sp(lo, hi);
            let rw = rewrite_reorderable_items(&self.get_context(), items, self.shape(), span);
            self.push_rewrite(span, rw);
        } else {
            for item in items {
                self.push_rewrite(item.span, None);
            }
        }

        item_length
    }

    /// Visit and format the given items. Items are reordered If they are
    /// consecutive and reorderable.
    pub fn visit_items_with_reordering(&mut self, mut items: &[&ast::Item]) {
        while !items.is_empty() {
            // If the next item is a `use`, `extern crate` or `mod`, then extract it and any
            // subsequent items that have the same item kind to be reordered within
            // `walk_reorderable_items`. Otherwise, just format the next item for output.
            let item_kind = ReorderableItemKind::from(items[0]);
            if item_kind.is_reorderable(self.config) {
                let visited_items_num =
                    self.walk_reorderable_items(items, item_kind, item_kind.in_group(self.config));
                let (_, rest) = items.split_at(visited_items_num);
                items = rest;
            } else {
                // Reaching here means items were not reordered. There must be at least
                // one item left in `items`, so calling `unwrap()` here is safe.
                let (item, rest) = items.split_first().unwrap();
                self.visit_item(item);
                items = rest;
            }
        }
    }
}
