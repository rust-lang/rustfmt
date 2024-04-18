use rustc_span::symbol::{sym, Ident};

use std::path::PathBuf;

use super::{FileModMap, ModResolver, Module};
use crate::attr::contains_name;
use crate::config::{Config, FileName};
use crate::parse::parser::{DirectoryOwnership, Parser};
use crate::parse::session::ParseSess;
use crate::utils::contains_skip;
use crate::Input;

fn validate_file_mod_map<F: Fn(&FileModMap<'_>)>(mod_path: &PathBuf, recursive: bool, f: F) {
    let config = Config::default();
    let input = Input::File(mod_path.clone());
    rustc_span::create_session_if_not_set_then(config.edition().into(), |_| {
        let parse_session = ParseSess::new(&config).unwrap();

        let directory_ownership = input
            .to_directory_ownership()
            .unwrap_or(DirectoryOwnership::UnownedViaBlock);
        let krate = Parser::parse_crate(input, &parse_session).unwrap();

        let mod_resolver = ModResolver::new(&parse_session, directory_ownership, recursive);
        let file_map = mod_resolver.visit_crate(&krate).unwrap();

        f(&file_map)
    })
}

fn get_submodule<'a>(module: &'a Module<'_>, mod_name: &str) -> &'a rustc_ast::ast::Item {
    get_nth_submodule(module, mod_name, 0)
}

fn get_nth_submodule<'a>(
    module: &'a Module<'_>,
    mod_name: &str,
    position: usize,
) -> &'a rustc_ast::ast::Item {
    module
        .items
        .iter()
        .filter(|i| i.ident == Ident::from_str(mod_name))
        .nth(position)
        .unwrap()
}

#[test]
fn external_sub_module_inner_attrs_are_present_in_mod_item_attrs_list() {
    // Ensure we can access external submodule inner attributes from the module items.
    //
    // Some inner attributes have formatting implications. For example, `#![rustfmt::skip]`
    // informs rustfmt not to format the module, and `#![macro_use]` informs rustfmt that it can't
    // safely reorder the module for fear of macro name collisions.

    let path = PathBuf::from("tests/mod-resolver/issue-4959-sub-mod-inner-attr/lib.rs");
    validate_file_mod_map(&path, true, |file_map| {
        let module = file_map.get(&FileName::Real(path.clone())).unwrap();

        let mod_a = get_submodule(module, "a");
        assert!(contains_name(&mod_a.attrs, sym::macro_use));

        let mod_b = get_submodule(module, "b");
        assert!(contains_name(&mod_b.attrs, sym::macro_use));

        // mod c is annotated with `#[rustfmt::skip]`, but we should still have access to
        // the inner attributes
        let mod_c = get_submodule(module, "c");
        assert!(contains_name(&mod_c.attrs, sym::macro_use));
        assert!(contains_skip(&mod_c.attrs));

        // mod d is annotated with an inner `#![rustfmt::skip]` attribute.
        let mod_d = get_submodule(module, "d");
        assert!(contains_skip(&mod_d.attrs));

        // mod e is defined in both e1.rs and e2.rs depending on the operating system.
        // We should ensure that attributes from one path aren't added to the other.
        let mod_e1 = get_nth_submodule(module, "e", 0);
        assert!(contains_name(&mod_e1.attrs, sym::no_std));
        assert!(!contains_name(&mod_e1.attrs, sym::no_implicit_prelude));

        let mod_e2 = get_nth_submodule(module, "e", 1);
        assert!(contains_name(&mod_e2.attrs, sym::no_implicit_prelude));
        assert!(!contains_name(&mod_e2.attrs, sym::no_std));
    });
}
