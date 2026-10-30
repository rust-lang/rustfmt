// rustfmt-file_lines: [{"file":"tests/source/issue-4053/method-chain.rs","range":[5,5]}]

fn main() {
    let diff = repo
        .diff_tree_to_workdir(Some(&head), Some(&mut diff_options))
        .unwrap();
}
