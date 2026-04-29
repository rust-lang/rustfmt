// rustfmt-file_lines: [{"file":"tests/source/issue-4053/long-chain-full.rs","range":[4,12]}]

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
