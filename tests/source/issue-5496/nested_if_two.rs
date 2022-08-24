// rustfmt-version: Two
// rustfmt-single_line_if_else_max_width: 75
fn foo() -> usize {
    // nested
    let _ = if true {
        if false { 1 } else { 2 }
    } else {
        3
    };
    let _ = if true {
        3
    } else {
        if false { 1 } else { 2 }
    };
    let _ = if true {
        if false { 1 } else { 2 }
    } else {
        if false { 3 } else { 4 }
    };

    1
}
