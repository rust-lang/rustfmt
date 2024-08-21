// rustfmt-style_edition: 2024
enum A {
    B {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    },

    #[multiline_macro_attribute(
        very_very_long_option1,
        very_very_long_option2,
        very_very_long_option3
    )]
    C {
        a: usize,
    },

    #[attr_with_expression1(x = ']')]
    D1 {
        a: usize,
    },

    #[attr_with_expression2(x = vec![])]
    D2 {
        a: usize,
    },

    #[attr_with_expression3(x = "]")]
    D3 {
        a: usize,
    },

    #[attr_with_expression4(x = "\"]")]
    D4 {
        a: usize,
    },

    #[attr1]
    #[attr2]
    D5 {
        a: usize,
    },
}
