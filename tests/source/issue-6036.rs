fmt_me!(
impl Foo { fn f(&self) {} }
);
fmt_me!(
impl Bar for Baz { fn m(&self) {} }
);

// does get formatted
fmt_me!(
impl<T> Foo<T> { fn g(&self) {} }
mod test {
impl Foo { fn f(&self) {} }
}
);
