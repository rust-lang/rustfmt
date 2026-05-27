// rustfmt-fn_parameter_post_comment_alignment: SingleSpace

fn foo(
    a: usize, // Chirp
    b: usize, // Bark
    c: f32, // Meow
) {
}

fn bar(
    a: usize, /* Chirp */
    b: usize, // Bark
    c: f32, /* Meow */
) {
    // `fn_parameter_post_comment_alignment` should not
    // affect post comments for match statements.
    match 0 {
        0 => todo!(), // IIIS
        1 => todo!(), // ASD
        _ => {}       // Meep
    }
}
