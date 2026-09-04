trait Foo {
    type Bar<'a>: Baz1 // comment
     where Self: 'a;
}

trait Qux {
    type Bar: Baz1 + Baz2 // comment
     where Self: Sized;
}
