//@ rustfmt-wrap_comments: true
//@ rustfmt-comment_width: 100

struct S {
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaaaaaa
    x: i32,
    /// This doc comment line is 101 columns long, so rustfmt wraps it onto a second line:
    /// aaaaaaaaaa
    y: i32,
    /** This block doc comment is 100 columns long, and should stay on a single line: aaaaaaaaaaa */
    block_100: i32,
    /** This block doc comment is 101 columns long, and should wrap onto a second line:
     * aaaaaaaaaa */
    block_101: i32,
    #[allow(dead_code)]
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaaaaaa
    after_attr: i32,
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaaaaaa
    #[allow(dead_code)]
    before_attr: i32,
}

struct TupleStruct(
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaaaaaa
    i32,
    /// This doc comment line is 101 columns long, so rustfmt wraps it onto a second line:
    /// aaaaaaaaaa
    i32,
);

union U {
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaaaaaa
    x: i32,
    /// This doc comment line is 101 columns long, so rustfmt wraps it onto a second line:
    /// aaaaaaaaaa
    y: u32,
}

enum E {
    StructVariant {
        /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaa
        x: i32,
        /// This doc comment line is 101 columns long, so rustfmt wraps it onto a second line:
        /// aaaaaa
        y: i32,
    },
    TupleVariant(
        /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line: aaaaaaaa
        i32,
        /// This doc comment line is 101 columns long, so rustfmt wraps it onto a second line:
        /// aaaaaa
        i32,
    ),
}
