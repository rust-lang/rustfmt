// rustfmt-style_edition: 2024
enum MyType {
    A { field1: bool, field2: bool },
    B { field1: bool, field2: bool },
    /// One-line doc comment
    C { field1: bool, field2: bool },
    /** Documentation block */
    D { field1: bool, field2: bool },
}
