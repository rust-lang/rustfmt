//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/method-chain.rs","range":[6,6]},{"file":"tests/source/file-lines/issue-4053/method-chain.rs","range":[11,11]}]

fn main() {
    // Selecting one method call in the chain.
    let diff = repo
        .diff_tree_to_workdir(Some(&head), Some(&mut diff_options))
        .unwrap();

    // Selecting the base of the chain.
    let diff = repo
        .diff_tree_to_workdir(Some(&head), Some(&mut diff_options))
        .unwrap();

    // Nothing selected.
    let diff =
    repo
    .diff_tree_to_workdir(  Some(&head),    Some(&mut diff_options)  )
    .unwrap  (  );
}
