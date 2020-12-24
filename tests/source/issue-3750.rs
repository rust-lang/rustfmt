// rustfmt-imports_merge_style: Crate

pub mod foo {
    pub mod bar {
        pub struct Bar;
    }

    pub fn bar() {}
}

use foo::bar;
use foo::bar::Bar;

fn main() {
    bar();
}
