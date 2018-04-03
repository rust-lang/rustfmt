fn foo() {
    match 0u32 {
        0 => (),
        _ => unreachable!(/* obviously */),
    }
}

fn foo() {
    let _ = column!(/* here */);
}
