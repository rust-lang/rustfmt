// rustfmt-indent_style: Visual

fn long_parent() {
    let bar = bazffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf().foo().bar().baz();

    bazffffffffffffffffffffffffffffffffffffasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfadfasdfasdfasdfadfasdfasdf().foo().foo().bar().baz();

}

fn long_first_child() {
    let foo = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();

    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();
}

fn long_tail() {
    bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");

    let foo = bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");
}
