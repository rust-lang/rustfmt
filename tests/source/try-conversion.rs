// rustfmt-use_try_shorthand: true

fn main() {
    let x = r#try!(some_expr());

    let y = r#try!(a.very.loooooooooooooooooooooooooooooooooooooong().chain().inside().weeeeeeeeeeeeeee()).test().0.x;
}

fn test() {
    a?
}

fn issue1291() {
    r#try!(fs::create_dir_all(&gitfiledir).chain_err(|| {
        format!("failed to create the {} submodule directory for the workarea",
                name)
    }));
}
