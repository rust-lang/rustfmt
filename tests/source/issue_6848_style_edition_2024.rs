// rustfmt-style_edition: 2024
// rustfmt-max_width: 80

#![feature(gen_blocks, try_blocks)]

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
        ) => unsafe {
        }
        _ => {}
    }

    // const block
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => const {
        }
        _ => {}
    } 
    
    // async block
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => async {
        },
        _ => {}
    }

    // gen block
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => gen {
        },
        _ => {}
    }

    // try block
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => try {
        },
        _ => {}
    }
}

fn main() {}