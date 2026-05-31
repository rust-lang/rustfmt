// rustfmt-post_comment_alignment: SingleSpace

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
    match 0 {
        0 => todo!(), // Beep
        1 => todo!(), // Boop
        _ => {} // Meep
    }
}
