// rustfmt-wrap_comments: true
// rustfmt-comment_width: 100

pub struct Foo {
    // This line has 99 characters ...............................................................9
    pub foo: u8,

    // This line has 100 characters ...............................................................9
    pub bar: u8,

    // This line has 101 characters
    // ................................................................9
    pub baz: u8,
}

pub mod foo {
    // This line has 100 characters ...............................................................9
    pub fn foo() {}
}
