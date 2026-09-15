// rustfmt-file_lines: [{"file":"tests/source/issue-4053/root-absorption.rs","range":[4,4]}]

fn method_chain(val: Option<i32>) {
        val
    .map(|val| val)
        .unwrap();
}
