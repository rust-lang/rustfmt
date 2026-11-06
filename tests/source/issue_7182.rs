// #7182: a comment inside empty generic arguments must survive formatting in a macro call.
macro_rules! m { ($f:ident </*c*/>) => {} }
macro_rules! n { ($f:ident <>) => {} }
fn main() {
    m!(Foo</*c*/>);
    m!(Foo</* a > b */>);
    m!(Foo</*c*/ >);
    n!(Foo<>);
    n!(Foo< >);
}
