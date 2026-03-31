// rustfmt-file_lines: [{"file":"tests/source/issue-4053.rs","range":[5,5]},{"file":"tests/source/issue-4053.rs","range":[11,11]}]

fn method_chain(val: Option<i32>) {
    let _ = val
    .map(|val| val)
        .unwrap();
}

fn match_expr(x: i32) {
    match x {
    0 => {},
        _ => {},
    };
}
