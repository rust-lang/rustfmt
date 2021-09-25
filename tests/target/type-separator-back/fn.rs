// rustfmt-type_separator: Back

fn myfunction1<
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
>(
    a: T1,
    b: T2,
) {
    //
}

fn myfunction2<T1, T2>(a: T1, b: T2)
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
