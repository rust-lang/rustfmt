const TRAIT_METHODS: [(&'static str, usize, SelfKind, OutType, &'static str); 30] = [
    ("add", 2, ValueSelf, AnyType, "std::ops::Add"),
    ("sub", 2, ValueSelf, AnyType, "std::ops::Sub"),
];
