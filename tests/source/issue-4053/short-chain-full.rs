// rustfmt-file_lines: [{"file":"tests/source/issue-4053/short-chain-full.rs","range":[4,5]}]

fn method_chain(val: Option<i32>) {
    let _ = val.map(|val| val)
        .unwrap();
}
