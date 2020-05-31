impl Type {
    #[rustfmt::skip::macros(macros)]
    fn example() {
        func(
            "format me"
        );
        other!(
            "format me"
        );
        macros!(
            "do not format"
        );
        macros![
            "do not format"
        ];
    }
}

impl Trait for Type {
    #[rustfmt::skip::macros(macros)]
    fn example() {
        func(
            "format me"
        );
        other!(
            "format me"
        );
        macros!(
            "do not format"
        );
    }
}

trait Trait {
    #[rustfmt::skip::macros(macros)]
    fn example() {
        func(
            "format me"
        );
        other!(
            "format me"
        );
        macros!(
            "do not format"
        );
    }
}
