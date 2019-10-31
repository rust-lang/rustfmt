// rustfmt-indent_style: Visual

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
    let bar = bazffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf()
              .foo()
              .bar()
              .baz();

    bazffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf()
    .foo()
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

fn long_first_child() {
    // Args that do not fit
    let bar = foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
                   .foo()
                   .bar()
                   .baz();

    foo().baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
         .foo()
         .bar()
         .baz();

    // Long element no args
    let foo = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
                   .foo()
                   .bar()
                   .baz();

    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
         .foo()
         .bar()
         .baz();

    // Long element with args that fit
    let bar = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
                   .foo()
                   .bar()
                   .baz();

    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
         .foo()
         .bar()
         .baz();
}

fn long_inner_child() {
    // Args that do not fit
    let bar = foo().foo_bar
                   .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
                   .foo()
                   .bar()
                   .baz();

    foo().foo_bar
         .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf")
         .foo()
         .bar()
         .baz();

    // Long element no args
    let foo = bar().foo_bar
                   .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
                   .foo()
                   .bar()
                   .baz();

    qux().foo_bar
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff()
         .foo()
         .bar()
         .baz();

    // Long element with args that fit
    let bar = bar().foo_bar
                   .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
                   .foo()
                   .bar()
                   .baz();

    qux().foo_bar
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff("abc")
         .foo()
         .bar()
         .baz();
}

fn long_tail() {
    // Args that do not fit
    let bar = foo().foo_bar
                   .foo()
                   .bar()
                   .baz()
                   .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf");

    foo().foo_bar
         .foo()
         .bar()
         .baz()
         .baz("ffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf");

    // Log element no args
    let foo = bar().foo_bar
                   .foo()
                   .bar()
                   .baz()
                   .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff();

    qux().foo_bar
         .foo()
         .bar()
         .baz()
         .asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff();

    // Long element with args that fit
    bar().xxxxxxx
         .map(|x| x + 5)
         .map(|x| x / 2)
         .fold(0, |acc, x| acc + x)
         .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");

    let foo = bar().xxxxxxx
                   .map(|x| x + 5)
                   .map(|x| x / 2)
                   .fold(0, |acc, x| acc + x)
                   .doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");
}
