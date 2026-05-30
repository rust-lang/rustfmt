// rustfmt-file_lines: [{"file":"tests/source/file-lines-where-clause.rs","range":[5,5]}]

fn foo<T, U, V>()
where
    T     :     Clone      +     Debug,
    U    :       Copy,
    V   :        Default,
{
}
