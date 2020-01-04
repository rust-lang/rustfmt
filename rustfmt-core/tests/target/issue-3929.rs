fn foo(x: T)
where
    for<U> T<U>: std::fmt::Debug,
{
    panic!()
}

macro_rules! m {
    (for<$T:ident> $($ty:ty),* $(,)?) => {};
}

m!(for<T> Box<T>, Arc<T>, Rc<T>);
