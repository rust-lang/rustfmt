// rustfmt-wrap_comments: true

/// This is a long line that angers rustfmt. Rustfmt shall deal with it swiftly and justly.
pub mod foo {}

fn chains() {
    test().map(|| {
        let x = 11;
        /* Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec a diam lectus. Sed sit amet ipsum mauris. Maecenas congue
         * ligula ac quam */ x
    });
}
