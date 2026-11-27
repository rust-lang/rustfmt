//@ rustfmt-style_edition: 2024
//@ rustfmt-max_width: 80

// characterization test for issue #6642 in SE 2024
struct S {
    f: (
        TypeIsTooLongToFitOnOneLineAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
    ),
    f: (u32),
    f: (u32),
    f: (u32),
    f: (MyType),
    f: (MyType),
    f: (MyType),
    f: (u32),
    f: (u32),
    f: (MyType),
    f: (MyType),
    f: (MyType),
    f: (MyType),
}
