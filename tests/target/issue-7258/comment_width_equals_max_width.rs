//@ rustfmt-wrap_comments: true
//@ rustfmt-max_width: 80
//@ rustfmt-comment_width: 80

struct S {
    /// This line is 80 columns long, so rustfmt shouldn't wrap it:
    /// aaaaaaaaaaaa
    x: i32,
    /// This line is 81 columns long, so rustfmt should wrap it:
    /// aaaaaaaaaaaaaaaa
    y: i32,
}

fn struct_lit() -> S {
    S {
        /// This line is 80 columns long, so rustfmt shouldn't wrap it:
        /// aaaaaaaa
        x: 1,
        /// This line is 81 columns long, so rustfmt should wrap it:
        /// aaaaaaaaaaaa
        y: 2,
    }
}
