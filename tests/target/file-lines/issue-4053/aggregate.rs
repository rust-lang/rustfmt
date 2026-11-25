//@ rustfmt-file_lines: [{"file":"tests/source/file-lines/issue-4053/aggregate.rs","range":[7,7]},{"file":"tests/source/file-lines/issue-4053/aggregate.rs","range":[14,14]},{"file":"tests/source/file-lines/issue-4053/aggregate.rs","range":[21,21]},{"file":"tests/source/file-lines/issue-4053/aggregate.rs","range":[27,27]},{"file":"tests/source/file-lines/issue-4053/aggregate.rs","range":[38,38]}]

fn main() {
    // Selecting one element of an array.
    let x = [1 + 1, 2 + 2, 3 + 3];

    // Selecting one element of a tuple.
    let x = (1 + 1, 2 + 2, 3 + 3);

    // Selecting one field of a struct.
    let x = Foo {
        a: 1 + 1,
        b: 2 + 2,
        c: 3 + 3,
    };

    // Selecting the struct name.
    let x = Foo {
        a: 1 + 1,
        b: 2 + 2,
        c: 3 + 3,
    };

    // Selecting the base in struct update syntax.
    let x = Foo {
        a: 1 + 1,
        b: 2 + 2,
        ..base
    };

    // Nothing selected.
    let x = [
    1  +  1  ,
    2  +  2  ,
    3  +  3  ,
    ];
    let x = (
    1  +  1  ,
    2  +  2  ,
    3  +  3  ,
    );
    let x = Foo {
    a  :  1  +  1  ,
    b  :  2  +  2  ,
    ..  base
    };
}
