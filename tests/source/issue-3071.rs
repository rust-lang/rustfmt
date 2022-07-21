// rustfmt-indent_style: Visual

struct A<T>
    where T: Send
{
    x: u32,
}

impl<T> A<T> where T: Send
{
    fn foo() {}
}
