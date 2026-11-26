//@ rustfmt-wrap_comments: true
//@ rustfmt-comment_width: 100
//@ rustfmt-struct_field_align_threshold: 20

struct Simple {
    x:            i32,
    longer_field: i32,
}

fn struct_lit() -> Simple {
    Simple {
        /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line:
        /// aaaaaaaa
        x:            1,
        /// This line is 101 columns long, so rustfmt should wrap it onto a second line:
        /// aaaaaaaaaaaa
        longer_field: 2,
    }
}
