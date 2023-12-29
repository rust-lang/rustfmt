// rustfmt-enum_variant_layout: Vertical

pub enum MultiAndSingleLine {
    A { field1: () },
    #[attr]
    B { field1: (), field2: (), },
}

enum SingleLine {
    A { field: () },
    B { test: () },
}