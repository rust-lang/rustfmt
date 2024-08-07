// rustfmt-version: Two

pub trait A {}
pub trait B {}
pub trait C {}

pub trait Foo:
// A and C
A + C
// and B
    + B
{}
