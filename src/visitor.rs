// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::{ast, ptr, visit};
use syntax::codemap::{self, CodeMap, Span, BytePos};
use syntax::parse::ParseSess;

use strings::string_buffer::StringBuffer;

use Indent;
use utils;
use codemap::{LineRangeUtils, SpanUtils};
use config::Config;
use rewrite::{Rewrite, RewriteContext};
use comment::rewrite_comment;
use macros::rewrite_macro;
use items::{rewrite_static, rewrite_associated_type, rewrite_type_alias, format_impl, format_trait};
use std::cmp::Ordering;

// For format_missing and last_pos, need to use the source callsite (if applicable).
// Required as generated code spans aren't guaranteed to follow on from the last span.
macro_rules! source {
    ($this:ident, $sp: expr) => {
        $this.codemap.source_callsite($sp)
    }
}

fn path_of(a: &ast::ViewPath_) -> &ast::Path {
    match a {
        &ast::ViewPath_::ViewPathSimple(_, ref p) => p,
        &ast::ViewPath_::ViewPathGlob(ref p) => p,
        &ast::ViewPath_::ViewPathList(ref p, _) => p,
    }
}

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

fn compare_path_list_items(a: &ast::PathListItem, b: &ast::PathListItem) -> Ordering {
    let name_ordering = match a.node.name() {
        Some(a_name) => {
            match b.node.name() {
                Some(b_name) => a_name.name.as_str().cmp(&b_name.name.as_str()),
                None => Ordering::Greater,
            }
        }
        None => {
            match b.node.name() {
                Some(_) => Ordering::Less,
                None => Ordering::Equal,
            }
        }
    };
    if name_ordering == Ordering::Equal {
        match a.node.rename() {
            Some(a_rename) => {
                match b.node.rename() {
                    Some(b_rename) => a_rename.name.as_str().cmp(&b_rename.name.as_str()),
                    None => Ordering::Greater,
                }
            }
            None => {
                match b.node.name() {
                    Some(_) => Ordering::Less,
                    None => Ordering::Equal,
                }
            }
        }
    } else {
        name_ordering
    }
}

fn compare_path_list_item_lists(a_items: &Vec<ast::PathListItem>,
                                b_items: &Vec<ast::PathListItem>)
                                -> Ordering {
    let mut a = a_items.clone();
    let mut b = b_items.clone();
    a.sort_by(|a, b| compare_path_list_items(a, b));
    b.sort_by(|a, b| compare_path_list_items(a, b));
    for comparison_pair in a.iter().zip(b.iter()) {
        let ord = compare_path_list_items(comparison_pair.0, comparison_pair.1);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    a.len().cmp(&b.len())
}

fn compare_view_path_types(a: &ast::ViewPath_, b: &ast::ViewPath_) -> Ordering {
    use syntax::ast::ViewPath_::*;
    match (a, b) {
        (&ViewPathSimple(..), &ViewPathSimple(..)) => Ordering::Equal,
        (&ViewPathSimple(..), _) => Ordering::Less,
        (&ViewPathGlob(_), &ViewPathSimple(..)) => Ordering::Greater,
        (&ViewPathGlob(_), &ViewPathGlob(_)) => Ordering::Equal,
        (&ViewPathGlob(_), &ViewPathList(..)) => Ordering::Less,
        (&ViewPathList(_, ref a_items), &ViewPathList(_, ref b_items)) => {
            compare_path_list_item_lists(a_items, b_items)
        }
        (&ViewPathList(..), _) => Ordering::Greater,
    }
}

fn compare_view_paths(a: &ast::ViewPath_, b: &ast::ViewPath_) -> Ordering {
    match compare_paths(path_of(a), path_of(b)) {
        Ordering::Equal => compare_view_path_types(a, b),
        cmp => cmp,
    }
}

fn is_use_item(item: &ast::Item) -> bool {
    match item.node {
        ast::ItemKind::Use(_) => true,
        _ => false,
    }
}
fn compare_use_items(a: &ast::Item, b: &ast::Item) -> Option<Ordering> {
    match (&a.node, &b.node) {
        (&ast::ItemKind::Use(ref a_vp), &ast::ItemKind::Use(ref b_vp)) => {
            Some(compare_view_paths(&a_vp.node, &b_vp.node))
        }
        _ => None,
    }
}

pub struct FmtVisitor<'a> {
    pub parse_session: &'a ParseSess,
    pub codemap: &'a CodeMap,
    pub buffer: StringBuffer,
    pub last_pos: BytePos,
    // FIXME: use an RAII util or closure for indenting
    pub block_indent: Indent,
    pub config: &'a Config,
}

impl<'a> FmtVisitor<'a> {
    fn visit_stmt(&mut self, stmt: &ast::Stmt) {
        debug!("visit_stmt: {:?} {:?}",
               self.codemap.lookup_char_pos(stmt.span.lo),
               self.codemap.lookup_char_pos(stmt.span.hi));

        // FIXME(#434): Move this check to somewhere more central, eg Rewrite.
        if !self.config.file_lines.contains(&self.codemap.lookup_line_range(stmt.span)) {
            return;
        }

        match stmt.node {
            ast::StmtKind::Decl(ref decl, _) => {
                if let ast::DeclKind::Item(ref item) = decl.node {
                    self.visit_item(item);
                } else {
                    let rewrite = stmt.rewrite(&self.get_context(),
                                               self.config.max_width - self.block_indent.width(),
                                               self.block_indent);

                    self.push_rewrite(stmt.span, rewrite);
                }
            }
            ast::StmtKind::Expr(..) |
            ast::StmtKind::Semi(..) => {
                let rewrite = stmt.rewrite(&self.get_context(),
                                           self.config.max_width - self.block_indent.width(),
                                           self.block_indent);

                self.push_rewrite(stmt.span, rewrite);
            }
            ast::StmtKind::Mac(ref mac, _macro_style, _) => {
                self.format_missing_with_indent(source!(self, stmt.span).lo);
                self.visit_mac(mac, None);
            }
        }
    }

    pub fn visit_block(&mut self, b: &ast::Block) {
        debug!("visit_block: {:?} {:?}",
               self.codemap.lookup_char_pos(b.span.lo),
               self.codemap.lookup_char_pos(b.span.hi));

        // Check if this block has braces.
        let snippet = self.snippet(b.span);
        let has_braces = snippet.starts_with("{") || snippet.starts_with("unsafe");
        let brace_compensation = if has_braces { BytePos(1) } else { BytePos(0) };

        self.last_pos = self.last_pos + brace_compensation;
        self.block_indent = self.block_indent.block_indent(self.config);
        self.buffer.push_str("{");

        for stmt in &b.stmts {
            self.visit_stmt(&stmt)
        }

        if let Some(ref e) = b.expr {
            self.format_missing_with_indent(source!(self, e.span).lo);
            let rewrite = e.rewrite(&self.get_context(),
                         self.config.max_width - self.block_indent.width(),
                         self.block_indent)
                .unwrap_or_else(|| self.snippet(e.span));

            self.buffer.push_str(&rewrite);
            self.last_pos = source!(self, e.span).hi;

            if utils::semicolon_for_expr(e) {
                self.buffer.push_str(";");
            }
        }

        // FIXME: we should compress any newlines here to just one
        self.format_missing_with_indent(source!(self, b.span).hi - brace_compensation);
        self.close_block();
        self.last_pos = source!(self, b.span).hi;
    }

    // FIXME: this is a terrible hack to indent the comments between the last
    // item in the block and the closing brace to the block's level.
    // The closing brace itself, however, should be indented at a shallower
    // level.
    fn close_block(&mut self) {
        let total_len = self.buffer.len;
        let chars_too_many = if self.config.hard_tabs {
            1
        } else {
            self.config.tab_spaces
        };
        self.buffer.truncate(total_len - chars_too_many);
        self.buffer.push_str("}");
        self.block_indent = self.block_indent.block_unindent(self.config);
    }

    // Note that this only gets called for function definitions. Required methods
    // on traits do not get handled here.
    fn visit_fn(&mut self,
                fk: visit::FnKind,
                fd: &ast::FnDecl,
                b: &ast::Block,
                s: Span,
                _: ast::NodeId,
                defaultness: ast::Defaultness) {
        let indent = self.block_indent;
        let rewrite = match fk {
            visit::FnKind::ItemFn(ident, ref generics, unsafety, constness, abi, vis) => {
                self.rewrite_fn(indent,
                                ident,
                                fd,
                                generics,
                                unsafety,
                                constness,
                                defaultness,
                                abi,
                                vis,
                                codemap::mk_sp(s.lo, b.span.lo),
                                &b)
            }
            visit::FnKind::Method(ident, ref sig, vis) => {
                self.rewrite_fn(indent,
                                ident,
                                fd,
                                &sig.generics,
                                sig.unsafety,
                                sig.constness,
                                defaultness,
                                sig.abi,
                                vis.unwrap_or(&ast::Visibility::Inherited),
                                codemap::mk_sp(s.lo, b.span.lo),
                                &b)
            }
            visit::FnKind::Closure => None,
        };

        if let Some(fn_str) = rewrite {
            self.format_missing_with_indent(source!(self, s).lo);
            self.buffer.push_str(&fn_str);
            if let Some(c) = fn_str.chars().last() {
                if c == '}' {
                    self.last_pos = source!(self, b.span).hi;
                    return;
                }
            }
        } else {
            self.format_missing(source!(self, b.span).lo);
        }

        self.last_pos = source!(self, b.span).lo;
        self.visit_block(b)
    }

    fn visit_item(&mut self, item: &ast::Item) {
        // This is where we bail out if there is a skip attribute. This is only
        // complex in the module case. It is complex because the module could be
        // in a seperate file and there might be attributes in both files, but
        // the AST lumps them all together.
        match item.node {
            ast::ItemKind::Mod(ref m) => {
                let outer_file = self.codemap.lookup_char_pos(item.span.lo).file;
                let inner_file = self.codemap.lookup_char_pos(m.inner.lo).file;
                if outer_file.name == inner_file.name {
                    // Module is inline, in this case we treat modules like any
                    // other item.
                    if self.visit_attrs(&item.attrs) {
                        self.push_rewrite(item.span, None);
                        return;
                    }
                } else if utils::contains_skip(&item.attrs) {
                    // Module is not inline, but should be skipped.
                    return;
                } else {
                    // Module is not inline and should not be skipped. We want
                    // to process only the attributes in the current file.
                    let attrs = item.attrs
                        .iter()
                        .filter_map(|a| {
                            let attr_file = self.codemap.lookup_char_pos(a.span.lo).file;
                            if attr_file.name == outer_file.name {
                                Some(a.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    // Assert because if we should skip it should be caught by
                    // the above case.
                    assert!(!self.visit_attrs(&attrs));
                }
            }
            _ => {
                if self.visit_attrs(&item.attrs) {
                    self.push_rewrite(item.span, None);
                    return;
                }
            }
        }

        match item.node {
            ast::ItemKind::Use(ref vp) => {
                self.format_import(&item.vis, vp, item.span);
            }
            ast::ItemKind::Impl(..) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                if let Some(impl_str) = format_impl(&self.get_context(), item, self.block_indent) {
                    self.buffer.push_str(&impl_str);
                    self.last_pos = source!(self, item.span).hi;
                }
            }
            ast::ItemKind::Trait(..) => {
                self.format_missing_with_indent(item.span.lo);
                if let Some(trait_str) = format_trait(&self.get_context(),
                                                      item,
                                                      self.block_indent) {
                    self.buffer.push_str(&trait_str);
                    self.last_pos = source!(self, item.span).hi;
                }
            }
            ast::ItemKind::ExternCrate(_) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                let new_str = self.snippet(item.span);
                self.buffer.push_str(&new_str);
                self.last_pos = source!(self, item.span).hi;
            }
            ast::ItemKind::Struct(ref def, ref generics) => {
                let rewrite = {
                    let indent = self.block_indent;
                    let context = self.get_context();
                    ::items::format_struct(&context,
                                           "struct ",
                                           item.ident,
                                           &item.vis,
                                           def,
                                           Some(generics),
                                           item.span,
                                           indent,
                                           None)
                        .map(|s| {
                            match *def {
                                ast::VariantData::Tuple(..) => s + ";",
                                _ => s,
                            }
                        })
                };
                self.push_rewrite(item.span, rewrite);
            }
            ast::ItemKind::Enum(ref def, ref generics) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                self.visit_enum(item.ident, &item.vis, def, generics, item.span);
                self.last_pos = source!(self, item.span).hi;
            }
            ast::ItemKind::Mod(ref module) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                self.format_mod(module, &item.vis, item.span, item.ident);
            }
            ast::ItemKind::Mac(ref mac) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                self.visit_mac(mac, Some(item.ident));
            }
            ast::ItemKind::ForeignMod(ref foreign_mod) => {
                self.format_missing_with_indent(source!(self, item.span).lo);
                self.format_foreign_mod(foreign_mod, item.span);
            }
            ast::ItemKind::Static(ref ty, mutability, ref expr) => {
                let rewrite = rewrite_static("static",
                                             &item.vis,
                                             item.ident,
                                             ty,
                                             mutability,
                                             Some(expr),
                                             &self.get_context());
                self.push_rewrite(item.span, rewrite);
            }
            ast::ItemKind::Const(ref ty, ref expr) => {
                let rewrite = rewrite_static("const",
                                             &item.vis,
                                             item.ident,
                                             ty,
                                             ast::Mutability::Immutable,
                                             Some(expr),
                                             &self.get_context());
                self.push_rewrite(item.span, rewrite);
            }
            ast::ItemKind::DefaultImpl(..) => {
                // FIXME(#78): format impl definitions.
            }
            ast::ItemKind::Fn(ref decl, unsafety, constness, abi, ref generics, ref body) => {
                self.visit_fn(visit::FnKind::ItemFn(item.ident,
                                                    generics,
                                                    unsafety,
                                                    constness,
                                                    abi,
                                                    &item.vis),
                              decl,
                              body,
                              item.span,
                              item.id,
                              ast::Defaultness::Final)
            }
            ast::ItemKind::Ty(ref ty, ref generics) => {
                let rewrite = rewrite_type_alias(&self.get_context(),
                                                 self.block_indent,
                                                 item.ident,
                                                 ty,
                                                 generics,
                                                 &item.vis,
                                                 item.span);
                self.push_rewrite(item.span, rewrite);
            }
        }
    }

    pub fn visit_trait_item(&mut self, ti: &ast::TraitItem) {
        if self.visit_attrs(&ti.attrs) {
            return;
        }

        match ti.node {
            ast::TraitItemKind::Const(ref ty, ref expr_opt) => {
                let rewrite = rewrite_static("const",
                                             &ast::Visibility::Inherited,
                                             ti.ident,
                                             ty,
                                             ast::Mutability::Immutable,
                                             expr_opt.as_ref(),
                                             &self.get_context());
                self.push_rewrite(ti.span, rewrite);
            }
            ast::TraitItemKind::Method(ref sig, None) => {
                let indent = self.block_indent;
                let rewrite = self.rewrite_required_fn(indent, ti.ident, sig, ti.span);
                self.push_rewrite(ti.span, rewrite);
            }
            ast::TraitItemKind::Method(ref sig, Some(ref body)) => {
                self.visit_fn(visit::FnKind::Method(ti.ident, sig, None),
                              &sig.decl,
                              &body,
                              ti.span,
                              ti.id,
                              ast::Defaultness::Final);
            }
            ast::TraitItemKind::Type(ref type_param_bounds, _) => {
                let rewrite = rewrite_associated_type(ti.ident,
                                                      None,
                                                      Some(type_param_bounds),
                                                      &self.get_context(),
                                                      self.block_indent);
                self.push_rewrite(ti.span, rewrite);
            }
        }
    }

    pub fn visit_impl_item(&mut self, ii: &ast::ImplItem) {
        if self.visit_attrs(&ii.attrs) {
            return;
        }

        match ii.node {
            ast::ImplItemKind::Method(ref sig, ref body) => {
                self.visit_fn(visit::FnKind::Method(ii.ident, sig, Some(&ii.vis)),
                              &sig.decl,
                              body,
                              ii.span,
                              ii.id,
                              ii.defaultness);
            }
            ast::ImplItemKind::Const(ref ty, ref expr) => {
                let rewrite = rewrite_static("const",
                                             &ii.vis,
                                             ii.ident,
                                             ty,
                                             ast::Mutability::Immutable,
                                             Some(expr),
                                             &self.get_context());
                self.push_rewrite(ii.span, rewrite);
            }
            ast::ImplItemKind::Type(ref ty) => {
                let rewrite = rewrite_associated_type(ii.ident,
                                                      Some(ty),
                                                      None,
                                                      &self.get_context(),
                                                      self.block_indent);
                self.push_rewrite(ii.span, rewrite);
            }
            ast::ImplItemKind::Macro(ref mac) => {
                self.format_missing_with_indent(source!(self, ii.span).lo);
                self.visit_mac(mac, Some(ii.ident));
            }
        }
    }

    fn visit_mac(&mut self, mac: &ast::Mac, ident: Option<ast::Ident>) {
        // 1 = ;
        let width = self.config.max_width - self.block_indent.width() - 1;
        let rewrite = rewrite_macro(mac, ident, &self.get_context(), width, self.block_indent);

        if let Some(res) = rewrite {
            self.buffer.push_str(&res);
            self.last_pos = source!(self, mac.span).hi;
        }
    }

    fn push_rewrite(&mut self, span: Span, rewrite: Option<String>) {
        self.format_missing_with_indent(source!(self, span).lo);
        let result = rewrite.unwrap_or_else(|| self.snippet(span));
        self.buffer.push_str(&result);
        self.last_pos = source!(self, span).hi;
    }

    pub fn from_codemap(parse_session: &'a ParseSess, config: &'a Config) -> FmtVisitor<'a> {
        FmtVisitor {
            parse_session: parse_session,
            codemap: parse_session.codemap(),
            buffer: StringBuffer::new(),
            last_pos: BytePos(0),
            block_indent: Indent {
                block_indent: 0,
                alignment: 0,
            },
            config: config,
        }
    }

    pub fn snippet(&self, span: Span) -> String {
        match self.codemap.span_to_snippet(span) {
            Ok(s) => s,
            Err(_) => {
                println!("Couldn't make snippet for span {:?}->{:?}",
                         self.codemap.lookup_char_pos(span.lo),
                         self.codemap.lookup_char_pos(span.hi));
                "".to_owned()
            }
        }
    }

    // Returns true if we should skip the following item.
    pub fn visit_attrs(&mut self, attrs: &[ast::Attribute]) -> bool {
        if utils::contains_skip(attrs) {
            return true;
        }

        let outers: Vec<_> = attrs.iter()
            .filter(|a| a.node.style == ast::AttrStyle::Outer)
            .cloned()
            .collect();
        if outers.is_empty() {
            return false;
        }

        let first = &outers[0];
        self.format_missing_with_indent(source!(self, first.span).lo);

        let rewrite = outers.rewrite(&self.get_context(),
                     self.config.max_width - self.block_indent.width(),
                     self.block_indent)
            .unwrap();
        self.buffer.push_str(&rewrite);
        let last = outers.last().unwrap();
        self.last_pos = source!(self, last.span).hi;
        false
    }

    fn walk_mod_items(&mut self, m: &ast::Mod) {
        let mut items_left: &[ptr::P<ast::Item>] = &m.items;
        while !items_left.is_empty() {
            if self.config.reorder_imports.reorder_lines() && is_use_item(&*items_left[0]) {
                let use_item_length =
                    items_left.iter().take_while(|ppi| is_use_item(&***ppi)).count();
                let (use_items, rest) = items_left.split_at(use_item_length);
                self.format_imports(use_items);
                items_left = rest;
            } else {
                // `unwrap()` is safe here because we know `items_left`
                // has elements from the loop condition
                let (item, rest) = items_left.split_first().unwrap();
                self.visit_item(&item);
                items_left = rest;
            }
        }
    }

    fn format_mod(&mut self, m: &ast::Mod, vis: &ast::Visibility, s: Span, ident: ast::Ident) {
        // Decide whether this is an inline mod or an external mod.
        let local_file_name = self.codemap.span_to_filename(s);
        let is_internal = local_file_name == self.codemap.span_to_filename(source!(self, m.inner));

        self.buffer.push_str(&*utils::format_visibility(vis));
        self.buffer.push_str("mod ");
        self.buffer.push_str(&ident.to_string());

        if is_internal {
            self.buffer.push_str(" {");
            // Hackery to account for the closing }.
            let mod_lo = self.codemap.span_after(source!(self, s), "{");
            let body_snippet =
                self.snippet(codemap::mk_sp(mod_lo, source!(self, m.inner).hi - BytePos(1)));
            let body_snippet = body_snippet.trim();
            if body_snippet.is_empty() {
                self.buffer.push_str("}");
            } else {
                self.last_pos = mod_lo;
                self.block_indent = self.block_indent.block_indent(self.config);
                self.walk_mod_items(m);
                self.format_missing_with_indent(source!(self, m.inner).hi - BytePos(1));
                self.close_block();
            }
            self.last_pos = source!(self, m.inner).hi;
        } else {
            self.buffer.push_str(";");
            self.last_pos = source!(self, s).hi;
        }
    }

    pub fn format_separate_mod(&mut self, m: &ast::Mod) {
        let filemap = self.codemap.lookup_char_pos(source!(self, m.inner).lo).file;
        self.last_pos = filemap.start_pos;
        self.block_indent = Indent::empty();
        self.walk_mod_items(m);
        self.format_missing(filemap.end_pos);
    }

    fn format_imports(&mut self, use_items: &[ptr::P<ast::Item>]) {
        let mut last_pos =
            use_items.first().map(|p_i| p_i.span.lo - BytePos(1)).unwrap_or(self.last_pos);
        let prefix = codemap::mk_sp(self.last_pos, last_pos);
        let mut ordered_use_items = use_items.iter()
            .map(|p_i| {
                let new_item = (&*p_i, last_pos);
                last_pos = p_i.span.hi;
                new_item
            })
            .collect::<Vec<_>>();
        // Order the imports by view-path & other import path properties
        ordered_use_items.sort_by(|a, b| compare_use_items(a.0, b.0).unwrap());
        // First, output the span before the first import
        self.format_missing(prefix.hi);
        for ordered in ordered_use_items {
            // Fake out the formatter by setting `self.last_pos` to the appropriate location before
            // each item before visiting it.
            self.last_pos = ordered.1;
            self.visit_item(&ordered.0);
        }
        self.last_pos = last_pos;
    }

    fn format_import(&mut self, vis: &ast::Visibility, vp: &ast::ViewPath, span: Span) {
        let vis = utils::format_visibility(vis);
        let mut offset = self.block_indent;
        offset.alignment += vis.len() + "use ".len();
        // 1 = ";"
        match vp.rewrite(&self.get_context(),
                         self.config.max_width - offset.width() - 1,
                         offset) {
            Some(ref s) if s.is_empty() => {
                // Format up to last newline
                let prev_span = codemap::mk_sp(self.last_pos, source!(self, span).lo);
                let span_end = match self.snippet(prev_span).rfind('\n') {
                    Some(offset) => self.last_pos + BytePos(offset as u32),
                    None => source!(self, span).lo,
                };
                self.format_missing(span_end);
                self.last_pos = source!(self, span).hi;
            }
            Some(ref s) => {
                let s = format!("{}use {};", vis, s);
                self.format_missing_with_indent(source!(self, span).lo);
                self.buffer.push_str(&s);
                self.last_pos = source!(self, span).hi;
            }
            None => {
                self.format_missing_with_indent(source!(self, span).lo);
                self.format_missing(source!(self, span).hi);
            }
        }
    }

    pub fn get_context(&self) -> RewriteContext {
        RewriteContext {
            parse_session: self.parse_session,
            codemap: self.codemap,
            config: self.config,
            block_indent: self.block_indent,
        }
    }
}

impl<'a> Rewrite for [ast::Attribute] {
    fn rewrite(&self, context: &RewriteContext, _: usize, offset: Indent) -> Option<String> {
        let mut result = String::new();
        if self.is_empty() {
            return Some(result);
        }
        let indent = offset.to_string(context.config);

        for (i, a) in self.iter().enumerate() {
            let mut a_str = context.snippet(a.span);

            // Write comments and blank lines between attributes.
            if i > 0 {
                let comment = context.snippet(codemap::mk_sp(self[i - 1].span.hi, a.span.lo));
                // This particular horror show is to preserve line breaks in between doc
                // comments. An alternative would be to force such line breaks to start
                // with the usual doc comment token.
                let multi_line = a_str.starts_with("//") && comment.matches('\n').count() > 1;
                let comment = comment.trim();
                if !comment.is_empty() {
                    let comment = try_opt!(rewrite_comment(comment,
                                                           false,
                                                           context.config.ideal_width -
                                                           offset.width(),
                                                           offset,
                                                           context.config));
                    result.push_str(&indent);
                    result.push_str(&comment);
                    result.push('\n');
                } else if multi_line {
                    result.push('\n');
                }
                result.push_str(&indent);
            }

            if a_str.starts_with("//") {
                a_str = try_opt!(rewrite_comment(&a_str,
                                                 false,
                                                 context.config.ideal_width - offset.width(),
                                                 offset,
                                                 context.config));
            }

            // Write the attribute itself.
            result.push_str(&a_str);

            if i < self.len() - 1 {
                result.push('\n');
            }
        }

        Some(result)
    }
}
