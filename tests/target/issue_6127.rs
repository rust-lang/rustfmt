// rustfmt-version: Two

trait Foo:
    Fn(
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
    ) -> ReallyLongTypeName
{
}

trait Bar:
    Fn(
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
    ) -> ReallyLongTypeName
    + Debug
    + Clone
{
}

trait FooBar:
    Clone
    + Debug
    + Fn(
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
        ReallyLongTypeName,
    ) -> ReallyLongTypeName
{
}
