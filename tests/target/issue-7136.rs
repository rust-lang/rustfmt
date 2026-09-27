macro_rules! ident_empty {
    ($f:ident <>) => {};
}
macro_rules! path_empty {
    ($f:ident :: <>) => {};
}
macro_rules! call_empty {
    ($f:ident :: <> ($($a:tt)*)) => {};
}
macro_rules! qualified_empty {
    ($m:ident :: $n:ident <>) => {};
}
macro_rules! pair_empty {
    ($a:ident <>, $b:ident <>) => {};
}
macro_rules! as_empty {
    ($a:ident as $t:ident <>) => {};
}

fn main() {
    ident_empty!(log<>);
    path_empty!(log::<>);
    call_empty!(S::<>());
    qualified_empty!(N::Foo<>);
    ident_empty!(Foo< >);
    pair_empty!(log<>, foo<>);
    as_empty!(a as Foo<>);
}
