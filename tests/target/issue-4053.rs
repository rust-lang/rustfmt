// rustfmt-file_lines: [{"file":"tests/source/issue-4053.rs","range":[8,8]},{"file":"tests/source/issue-4053.rs","range":[17,17]},{"file":"tests/source/issue-4053.rs","range":[23,23]},{"file":"tests/source/issue-4053.rs","range":[36,36]}]

fn method_chain(val: Option<i32>) {
    // Format one line of a short expression. If we were formatting the whole
    // expression we would put it all on one line, so we need to handle it
    // differently if we're formatting just one part of the expression.
    let _ = val
        .map(|val| val)
        .unwrap();

    // Format one method call in a large method chain.
    let _ = val
        .map(|val| val)
    .map(|val| val).map(|val| val)
        .map(|val| val)
        .map(|val| val)
        .map(|val| val)
        .map(|val| val)
            .map(|val| val)
        .unwrap();

    // Format the first part of a large method chain.
    let _ = val
        .map(|val| val)
        .map(|val| val)
        .map(|val| val)
    .map(|val| val).map(|val| val)
        .map(|val| val)
        .map(|val| val)
    .map(|val| val)
        .map(|val| val)
            .map(|val| val)
        .unwrap();
}

fn match_expr(x: i32) {
    match x {
        0 => {},
        _ => {},
    };
}
