//! Module that contains skip related stuffs.

use rustc_ast::ast::{Attribute, PathSegment};
use rustc_span::symbol::{sym, Symbol};

macro_rules! sym {
    ($tt:tt) => {
        Symbol::intern(stringify!($tt))
    };
}

/// Take care of skip name stack. You can update it by attributes slice or
/// by other context. Query this context to know if you need skip a block.
#[derive(Default, Clone)]
pub(crate) struct SkipContext {
    macros: Vec<String>,
    attributes: Vec<String>,
}

impl SkipContext {
    pub(crate) fn update_with_attrs(&mut self, attrs: &[Attribute]) {
        fn get_skip_names(vs: &mut Vec<String>, attr: &Attribute) {
            if let Some(list) = attr.meta_item_list() {
                for nested_meta_item in list {
                    if let Some(name) = nested_meta_item.ident() {
                        vs.push(name.to_string());
                    }
                }
            }
        }
        for attr in attrs {
            if let syntax::ast::AttrKind::Normal(ref attr_item) = &attr.kind {
                if is_skip_attr_with(&attr_item.path.segments, |s| s == sym!(macros)) {
                    get_skip_names(&mut self.macros, attr)
                } else if is_skip_attr_with(&attr_item.path.segments, |s| s == sym::attributes) {
                    get_skip_names(&mut self.attributes, attr)
                }
            }
        }
    }

    pub(crate) fn update(&mut self, mut other: Self) {
        self.macros.append(&mut other.macros);
        self.attributes.append(&mut other.attributes);
    }

    pub(crate) fn skip_macro(&self, name: &str) -> bool {
        self.macros.iter().any(|n| n == name)
    }

    pub(crate) fn skip_attribute(&self, name: &str) -> bool {
        self.attributes.iter().any(|n| n == name)
    }
}

/// Say if you're playing with `rustfmt`'s skip attribute
pub(crate) fn is_skip_attr(segments: &[PathSegment]) -> bool {
    is_skip_attr_with(segments, |s| s == sym!(macros) || s == sym::attributes)
}

fn is_skip_attr_with(segments: &[PathSegment], pred: impl FnOnce(Symbol) -> bool) -> bool {
    if segments.len() < 2 || segments[0].ident.name != sym::rustfmt {
        return false;
    }
    match segments.len() {
        2 => segments[1].ident.name == sym!(skip),
        3 => segments[1].ident.name == sym!(skip) && pred(segments[2].ident.name),
        _ => false,
    }
}
