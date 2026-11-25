//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/macro.rs","range":[6,6]},{"file":"tests/source/file-lines/issue-4053/macro.rs","range":[15,15]},{"file":"tests/source/file-lines/issue-4053/macro.rs","range":[21,21]}]

fn main() {
    // Selecting the macro name.
    let x = foo!(a, b + 1, c);

    // Selecting one argument.
    let x = foo!(a, b + 1, c);

    // Selecting one element of a bracketed macro call.
    let x = vec![1 + 1, 2 + 2, 3 + 3];

    // Nothing selected.
    let x =
    foo!
    (  a  ,
    b  +  1  ,
    c  );
    let x = vec![
    1  +  1  ,
    2  +  2  ,
    3  +  3  ,
    ];
}
