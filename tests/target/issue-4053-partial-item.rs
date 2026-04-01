// rustfmt-file_lines: [{"file":"tests/source/issue-4053-partial-item.rs","range":[7,7]},{"file":"tests/source/issue-4053-partial-item.rs","range":[12,13]}]

fn method_chain(val: Option<i32>) {
    let _ = val
        .map(|val| val)
    .map(|val| val).map(|val| val)
        .map(|val| {
                println!("...");
                val
            })
        .map(|val| val)
        .map(|val| {
            println!("...");
        val
    })
        .map(|val| val)
            .map(|val| val)
        .unwrap();
}
