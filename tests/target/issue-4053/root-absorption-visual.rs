// rustfmt-indent_style: Visual
// rustfmt-file_lines: [{"file":"tests/source/issue-4053/root-absorption-visual.rs","range":[5,5]}]

fn method_chain(val: Option<i32>) {
    val
    .map(|val| val)
        .unwrap();
}
