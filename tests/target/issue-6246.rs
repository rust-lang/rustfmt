fn main() {
    let foo =
        // 114514
        if true { 1919 } else { 810 };
}

// Test a let statement without equal sign
fn main() {
    let mut foo;
    // 114514
    foo = if true { 1919 } else { 810 };
}
