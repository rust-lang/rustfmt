#[derive(serde :: Serialize)]
enum Something {
    Variant
}

#[derive(
    serde :: Serialize,
    IReallyLoveToWriteLongDerives,
    Debug,
    Eq,
    PartialEq, Ord, PartialOrd,
    Hash, Clone, Copy, Default
)]
enum SomethingComplex {
    Variant5
}