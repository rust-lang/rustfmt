// rustfmt-style_edition: 2024
enum A {
    B {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    },

    #[attr]
    C {
        a: usize,
    },
}
