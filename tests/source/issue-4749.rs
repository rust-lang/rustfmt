struct S {
    x: (/* type */),
}

struct S2((/* type */));

struct S3((/* type
2 */));

struct S4((
    // Line comment
));

enum E {
    Variant1 {
        x: (/* type */),
    },
    Variant2((/* type */)),
}
