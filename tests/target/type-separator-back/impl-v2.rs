// rustfmt-type_separator: Back
// rustfmt-version: Two

impl<
    T1: Add +
        AddAssign +
        Clone +
        Copy +
        Debug +
        Default +
        Eq +
        Hash +
        Ord +
        PartialEq +
        PartialOrd +
        Sized,
    T2: Add +
        AddAssign +
        Clone +
        Copy +
        Debug +
        Default +
        Eq +
        Hash +
        Ord +
        PartialEq +
        PartialOrd +
        Sized,
> Foo<T1, T2> for MyType1
{
    //
}

impl<T1, T2> Foo<T1, T2> for MyType2
where
    T1: Add +
        AddAssign +
        Clone +
        Copy +
        Debug +
        Default +
        Eq +
        Hash +
        Ord +
        PartialEq +
        PartialOrd +
        Sized,
    T2: Add +
        AddAssign +
        Clone +
        Copy +
        Debug +
        Default +
        Eq +
        Hash +
        Ord +
        PartialEq +
        PartialOrd +
        Sized,
{
    //
}

fn myfunction() -> impl Add +
AddAssign +
Clone +
Copy +
Debug +
Default +
Eq +
Hash +
Ord +
PartialEq +
PartialOrd +
Send {
    5
}
