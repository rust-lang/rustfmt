fn test() {
    let foo = [
        (b"Welcome to Ringboard!" as &[u8], &em),
        (
            b"Ringboard is a fast, efficient, and composable clipboard manager for Linux."
          ,  &em,
        ),
        (
           b"It supports both Wayland and X11 along with multiple clients to manage your \
             clipboard history."
   ,         &em,
        ),
        (
            b"Clients include a standard GUI, an interactive TUI, and a CLI for all your scripting \
             needs."
           , &em,
        ),
        (
            "Ringboard can copy arbitrary bytes—that includes images!".as_bytes(),
            &em,
        ),
        (
            b"Plaintext and RegEx search are available for fast entry retrieval.",
            &em,
        ),
        (b"Enjoy this image from our AI overlords:", &em),
        (
            include_bytes!("../logo.jpeg") as &[u8],
            &MimeType::from("image/jpeg").unwrap(),
        ),
        (
            b"Finally, it's worth mentioning that Ringboard is extremely efficient, performant, \
             and scalable."
           , &em,
        ),
    ];
}
