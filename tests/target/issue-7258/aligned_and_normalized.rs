//@ rustfmt-wrap_comments: true
//@ rustfmt-comment_width: 100
//@ rustfmt-normalize_doc_attributes: true
//@ rustfmt-struct_field_align_threshold: 20

struct Aligned {
    /// This line is 100 columns long, so rustfmt shouldn't wrap it onto a second line:
    /// aaaaaaaaaaaa
    x:            i32,
    /// This comment line winds up 100 columns long, so rustfmt shouldn't wrap it:
    /// aaaaaaaaaaaaaaaaa
    longer_field: i32,
    /// This comment line winds up 101 columns long, so rustfmt should wrap it:
    /// aaaaaaaaaaaaaaaaaaaaa
    third:        i32,
}
