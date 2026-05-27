// rustfmt-fn_parameter_post_comment_alignment: SameIndent

fn foo(
    a: usize, // Chirp
    b: usize, // Bark
    c: f32,   // Meow
) {
}

fn bar(
    a: usize, /* Chirp */
    b: usize, // Bark
    c: f32,   /* Meow */
) {
}
