// rustfmt-indent_style: Block

// https://github.com/rust-lang/rustfmt/issues/3863
fn issue_3863() {
    foo("This text is under the max_width limit, and shouldn't cause any problems on its own.")
        .long("But this line is extra long, and doesn't fit within 100 max_width. 1234567890123456789 aBcDeFgHiJ")
        .baz()
        .collect()
        .unwrap();
}

fn long_parent() {
    // Args that do not fit
    let bar = baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    // Long element no args
    let foo = looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg()
        .foo()
        .bar()
        .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
        .foo()
        .bar()
        .baz();

    // Long element with args that fit
    let bar = looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooonnnnnnnnnnnnnnnnnnnnnnnnggggggggggggggggggggggggggggggggggggggggggg("ffffffffffffffffffffffffffffffffffff")
        .foo()
        .bar()
        .baz();

    asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("ffffffffffffffffffffffffffffffffffff")
        .foo()
        .bar()
        .baz();
}

fn long_inner() {
    // Args that do not fit
    let bar = bar()
        .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    qux()
        .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
        .foo()
        .bar()
        .baz();

    // Long element with args that fit
    let bar = bar()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
            "ffffffffffffffffffffffffffffffffffff",
        )
        .foo()
        .bar()
        .baz();

    qux()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
            "ffffffffffffffffffffffffffffffffffff",
        )
        .foo()
        .bar()
        .baz();

    // Long element no args
    let foo = bar()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
        )
        .foo()
        .bar()
        .baz();

    qux()
        .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff(
        )
        .foo()
        .bar()
        .baz();
}

fn long_tail() {
    // Args that do not fit
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .baz("fadfasdf39ru8ahsdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");

    // Long element with args that fit
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
            "abc123def456",
        );

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
            "abc123def456",
        );

    // Long element no args
    bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
        );

    let foo = bar()
        .xxxxxxx
        .map(|x| x + 5)
        .map(|x| x / 2)
        .fold(0, |acc, x| acc + x)
        .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff(
        );
}
