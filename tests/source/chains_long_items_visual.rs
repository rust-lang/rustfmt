// rustfmt-indent_style: Visual

fn long_inner() {
    let foo = bar().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();
    qux().asdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff().foo().bar().baz();
}

fn long_tail() {
    bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");
}

fn assignment_long_tail() {
let foo = bar().xxxxxxx.map(|x| x + 5).map(|x| x / 2).fold(0, |acc, x| acc + x).doooooooooooooooooooooooooooooooooooooooooooooooooooooo_stufffffffffffffffffffffffffffffffffffffff("abcdefghadfasdfasdfasdfasdfadf");

}
