struct Foo {
    bar: (),
    // Comment
}

struct Bar {
    baz: (),
    /*
    Comment
    */
}

struct Baz(
    (),
    // Comment
);

fn main() {
    let _ = Foo {
        bar: (),
        // Comment
    };

    let _ = Bar {
        baz: (),
        /*
        Comment
        */
    };

    let _ = Baz(
        (),
        // Comment
    );

    match a {
        0 => {}
        // Foo
        1 => {} // Bar
        // Baz
        2 => {}
        // Qux
    }
}
