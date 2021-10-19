// rustfmt-type_separator: Back

enum MyEnum1<T1: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized, T2: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized> {
    A(T1),
    B(T2),
}

enum MyEnum2<T1, T2> where T1: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized, T2: Add + AddAssign + Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + Sized {
    A(T1),
    B(T2),
}
