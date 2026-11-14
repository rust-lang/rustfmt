// rustfmt-indent_style: Visual
// rustfmt-where_single_line: true

struct A<T>
    where T: Send
{
    x: u32,
}

impl<T> A<T>
    where T: Send
{
    fn foo() {}
}

struct B<T, K>
    where T: Send,
          K: Eq
{
    y: u32,
}

impl<T, K> B<T, K>
    where T: Send,
          K: Eq
{
    fn bar() {}
}
