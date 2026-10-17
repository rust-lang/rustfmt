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

// Default settings: `fn_call_width` is 60. `max_width` is 100.
// First line string length is 59. Line length is 98.
pub const ABCDEFGHIJKLMN: &str = foo!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456
    next_line
");

// First line string length is 60. Line length is 98.
pub const ABCDEFGHIJKLM: &str = foo!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567
    next_line
");

// First line string length is 59. Line length is 99.
pub const ABCDEFGHIJKLMNO: &str = foo!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456
    next_line
");

// First line string length is 60. Line length is 99.
pub const ABCDEFGHIJKLMN: &str = foo!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz01234567
    next_line
");
