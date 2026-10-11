// rustfmt-style_edition: 2027

pub const MAC: &str = foo!("
    foo
    bar
");

pub const SHORT_MAC: &str = f!("
    foo
    bar
");

pub const LONG_MAC: &str = long_foo!("
    foo
    bar
");

pub const FUN: &str = foo("
    foo
    bar
");

pub const SHORT_FUN: &str = f("
    foo
    bar
");

pub const LONG_FUN: &str = long_foo("
    foo
    bar
");

pub const BAR: &str = bar!("
    foo bar
");

pub const BAZ: &str = baz!(
    "
    foo
    bar
"
);

fn main() {
    let bytes1 = hex!("
        00010203 04050607
        08090a0b 0c0d0e0f
    ");
}
