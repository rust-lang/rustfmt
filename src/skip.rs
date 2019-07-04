use syntax::ast;

#[derive(Default, Clone)]
pub(crate) struct SkipContext {
    macros: Vec<String>,
    attributes: Vec<String>,
}

impl SkipContext {
    pub(crate) fn update_by_attrs(&mut self, attrs: &[ast::Attribute]) {
        self.macros.append(&mut get_skip_names("macros", attrs));
        self.attributes
            .append(&mut get_skip_names("attributes", attrs));
    }

    pub(crate) fn update(&mut self, mut other: SkipContext) {
        self.macros.append(&mut other.macros);
        self.attributes.append(&mut other.attributes);
    }

    pub(crate) fn macro_skip(&self, name: &str) -> bool {
        self.macros.iter().any(|n| n == name)
    }

    pub(crate) fn attributes_skip(&self, name: &str) -> bool {
        self.attributes.iter().any(|n| n == name)
    }
}

static RUSTFMT: &'static str = "rustfmt";
static SKIP: &'static str = "skip";

pub(crate) fn is_skip_attr(segments: &[ast::PathSegment]) -> bool {
    if segments.len() < 2 || segments[0].ident.to_string() != RUSTFMT {
        return false;
    }
    match segments.len() {
        2 => segments[1].ident.to_string() == SKIP,
        3 => {
            segments[1].ident.to_string() == SKIP
                && ["macros", "attributes"]
                    .iter()
                    .any(|&n| n == &segments[2].ident.name.as_str())
        }
        _ => false,
    }
}

fn get_skip_names(kind: &str, attrs: &[ast::Attribute]) -> Vec<String> {
    let mut skip_names = vec![];
    let path = format!("{}::{}::{}", RUSTFMT, SKIP, kind);
    for attr in attrs {
        // syntax::ast::Path is implemented partialEq
        // but it is designed for segments.len() == 1
        if format!("{}", attr.path) != path {
            continue;
        }

        if let Some(list) = attr.meta_item_list() {
            for nested_meta_item in list {
                if let Some(name) = nested_meta_item.ident() {
                    skip_names.push(name.to_string());
                }
            }
        }
    }
    skip_names
}
