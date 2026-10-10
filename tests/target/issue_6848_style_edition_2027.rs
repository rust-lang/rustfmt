// rustfmt-style_edition: 2027
// rustfmt-max_width: 80

#![feature(gen_blocks, try_blocks)]

#[derive(Clone, Copy)]
enum ExampleTypeX {
    VariantAlphaSampleXYZ,
    VariantBetaXYZ,
    VariantABCD,
    VariantABCDE,
    VariantABCDEF,
    VariantABCDEFG,
    VariantABCDEFGH,
    VariantABCDEFGHI,
    VariantABCDEFGHIJ,
    VariantABCDEFGHIJK,
}

fn demo(lhs: ExampleTypeX, rhs: ExampleTypeX) {
    //unsafe block
    match (lhs, rhs) {
        (
            ExampleTypeX::VariantAlphaSampleXYZ,
            ExampleTypeX::VariantBetaXYZ,
        ) => unsafe {},
        _ => {}
    }

    match (lhs, rhs) {
        // 1 char below the max_width limit
        (ExampleTypeX::VariantABCD, ExampleTypeX::VariantBetaXYZ) => unsafe {},
        // everything properly fits on 1 line at exactly the max_width limit
        (ExampleTypeX::VariantABCDE, ExampleTypeX::VariantBetaXYZ) => unsafe {},
        // 1 char over the max_width limit
        (ExampleTypeX::VariantABCDEF, ExampleTypeX::VariantBetaXYZ) =>
            unsafe {},
        (ExampleTypeX::VariantABCDEF, ExampleTypeX::VariantBetaXYZ) => unsafe {
            non_empty_block()
        },
        (
            ExampleTypeX::VariantABCDEFGHIJKL,
            ExampleTypeX::VariantBetaXYZ,
        ) => unsafe { non_empty_block() },
        (
            ExampleTypeX::VariantABCDEFGHIJKLMNO,
            ExampleTypeX::VariantBetaXYZ,
        ) => unsafe { non_empty_block() },
        (ExampleTypeX::VariantABCDEFHGHI, ExampleTypeX::VariantBetaXYZ) =>
            'a: {}
        _ => {}
    }

    // const block
    match (lhs, rhs) {
        // 1 char below the max_width limit
        (ExampleTypeX::VariantABCDE, ExampleTypeX::VariantBetaXYZ) => const {},
        // everything properly fits on 1 line at exactly the max_width limit
        (ExampleTypeX::VariantABCDEF, ExampleTypeX::VariantBetaXYZ) => const {},
        // 1 char over the max_width limit
        (ExampleTypeX::VariantABCDEFG, ExampleTypeX::VariantBetaXYZ) => {
            const {}
        }
        _ => {}
    }

    // async block
    match (lhs, rhs) {
        // 1 char below the max_width limit
        (ExampleTypeX::VariantABCDE, ExampleTypeX::VariantBetaXYZ) => async {},
        // everything properly fits on 1 line at exactly the max_width limit
        (ExampleTypeX::VariantABCDEF, ExampleTypeX::VariantBetaXYZ) => async {},
        // 1 char over the max_width limit
        (ExampleTypeX::VariantABCDEFG, ExampleTypeX::VariantBetaXYZ) => {
            async {}
        }
        _ => {}
    }

    // gen block
    match (lhs, rhs) {
        // 1 char below the max_width limit
        (ExampleTypeX::VariantABCDEFG, ExampleTypeX::VariantBetaXYZ) => gen {},
        // everything properly fits on 1 line at exactly the max_width limit
        (ExampleTypeX::VariantABCDEFGH, ExampleTypeX::VariantBetaXYZ) => gen {},
        // 1 char over the max_width limit
        (ExampleTypeX::VariantABCDEFGHI, ExampleTypeX::VariantBetaXYZ) => {
            gen {}
        }
        _ => {}
    }

    // try block
    match (lhs, rhs) {
        //1 char below the max_width limit
        (ExampleTypeX::VariantABCDEFG, ExampleTypeX::VariantBetaXYZ) => try {},
        // everything properly fits on 1 line at exactly the max_width limit
        (ExampleTypeX::VariantABCDEFGH, ExampleTypeX::VariantBetaXYZ) => try {},
        // 1 char over the max_width limit
        (ExampleTypeX::VariantABCDEFGHI, ExampleTypeX::VariantBetaXYZ) => {
            try {}
        }
        _ => {}
    }
}

fn main() {}
