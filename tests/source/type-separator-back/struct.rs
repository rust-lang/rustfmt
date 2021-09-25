// rustfmt-type_separator: Back

struct MyStruct1<T1: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized, T2: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized> {
    a: T1,
    b: T2,
}

struct MyStruct2<T1, T2> where T1: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized, T2: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized {
    a: T1,
    b: T2,
}
