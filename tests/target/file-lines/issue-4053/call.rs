//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/call.rs","range":[6,6]},{"file":"tests/source/file-lines/issue-4053/call.rs","range":[15,15]}]

fn main() {
    // Selecting the function name.
    let x = foo::bar(a, b + 1, c);

    // Selecting one argument.
    let x = foo::bar(a, b + 1, c);

    // Nothing selected.
    let x =
    foo::bar
    (  a  ,
    b  +  1  ,
    c  );
}
