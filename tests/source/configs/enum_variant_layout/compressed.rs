// rustfmt-enum_variant_layout: Compressed

pub enum MultiAndSingleLine {
    A { field1: () },
    #[attr]
    B { field1 : (), field2: (), },
}

enum SingleLine {
    A { field: () },
    B { test: () },
}

enum MyType {
    A { field1: bool, field2: bool },
    B { field1: bool, field2: bool },
    /// OMG a comment
    C { field1: bool, field2: bool },
    D { field1: bool, field2: bool },
}