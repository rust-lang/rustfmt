// rustfmt-max_width: 80

#[derive(Clone, Copy)]
enum ExampleTypeX {
    VariantAlphaSampleXYZ,
    VariantBetaXYZ,
}

fn demo(lhs: ExampleTypeX, rhs: ExampleTypeX) {
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => unsafe {},
        _ => {}
    }
}

fn main() {}
