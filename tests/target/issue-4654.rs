// rustfmt-version: Two

// Struct comments that start in new line

// No trailing comma

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
        /*
        Comment
        */
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
}

// With trailing comma

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
        /*
        Comment
        */
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
}

// With new line before trailing comma

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
        /*
        Comment
        */
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
}
