// rustfmt-file_lines: [{"file":"tests/source/issue-4053/comments.rs","range":[10,10]}]

fn method_chain(val: Option<i32>) {
    let _ = val.map(|val| val).map(|val| val)
        .map(|val| val)
    // top comment
    .map(|val| val).map(|val| val)
        .map(|val| val)        // back comment
        .map(|val| val) // back comment
    .map(|val| val)
        .map(|val| val)
            .map(|val| val)
            // top comment

        .unwrap();
}
