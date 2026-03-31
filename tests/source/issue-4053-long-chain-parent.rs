// rustfmt-file_lines: [{"file":"tests/source/issue-4053-long-chain-parent.rs","range":[4,4]}]

fn method_chain(val: Option<i32>) {
    let _ = val.map(|val| val).map(|val| val)
        .map(|val| val)
    .map(|val| val).map(|val| val)
        .map(|val| val)
        .map(|val| val)
    .map(|val| val)
        .map(|val| val)
            .map(|val| val)
        .unwrap();
}
