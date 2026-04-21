// rustfmt-file_lines: [{"file":"tests/source/file-lines-where-clause-skip.rs","range":[8,8]}]

fn foo<T>() where
T: Clone,
T: Copy,
T: Default,
{
    let x = 1;
}
