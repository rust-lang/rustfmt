// rustfmt-style_edition: 2027

fn is_something(foo: Foo, bar: Bar) -> bool {
    matches!((legacy_required_finality, signature_weight),
        | (LegacyRequiredFinality::Any, Insufficient | Weak | Strict)
        | (LegacyRequiredFinality::Weak, Weak | Strict)
        | (LegacyRequiredFinality::Strict, Strict)
    )
}
