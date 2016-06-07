// rustfmt-use_try_shorthand: true
// rustfmt-chain_indent: Tabbed

fn main() {
    let x = try!(some_expr());

    let y = try!(a.very.loooooooooooooooooooooooooooooooooooooong().chain().inside().weeeeeeeeeeeeeee()).test().0.x;
}

fn test() {
    a?
}
