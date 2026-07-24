// rustfmt-file_lines: [{"file":"tests/source/issue-4053/inline-gap.rs","range":[7,7]}]

fn method_chain(val: Option<i32>) {
    let _ = val
        .map(|val| val)
    .map(|val| val)    .map(|val| val)
    .map(|val| val)
        .unwrap();
}
