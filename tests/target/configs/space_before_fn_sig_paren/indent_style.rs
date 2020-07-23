// rustfmt-space_before_fn_sig_paren: true
// rustfmt-indent_style: Visual
// rustfmt-max_width: 30
// Function space before function paren

fn foo () {
    // ...
}
fn foo_with_multi_lined (a: u32,
                         b: u32,
                         c: u32)
{
    // ...
}
fn foo<T> (bar: T) {
    // ...
}
fn foo<T> (a: T,
           b: u32,
           c: u32) {
    // ...
}
fn foo<T: Foo + Bar,
       F: FooBar> (
    a: T,
    b: u32,
    c: u32) {
    // ...
}
