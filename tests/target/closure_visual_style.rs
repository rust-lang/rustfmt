// rustfmt-indent_style: Visual
// rustfmt-normalize_comments: true

fn foo() {
    {
        let write_status = |status: &mut Vec<ansi_term::ANSIString>,
                            diff: &Diff,
                            heading: &str,
                            color: &Style,
                            show_hints: bool,
                            hints: &[&str]|
         -> Option<bool> { Some(true) };
    }
}

fn bar() {
    let write_status = |status: &mut Vec<ansi_term::ANSIString>,
                        diff: &Diff,
                        heading: &str,
                        color: &Style|
     -> Option<bool> { Some(true) };
    let baz = |foo: bool| -> Option<bool> { Some(true) };
}

fn main() {
    let square = (|i: i32| i * i);

    let commented = |// first
                     a, // argument
                     // second
                     b: WithType, // argument
                     // ignored
                     _| {
        (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb)
    };

    let commented2 =
        |// first
         a, // argument
         // second
         b: WithType, // argument
         // ignored
         _| (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb);

    let block_body = move |xxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
                           ref yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy| {
        xxxxxxxxxxxxxxxxxxxxxxxxxxxxx + yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
    };

    let loooooooooooooong_name = |field| {
        // format comments.
        if field.node.attrs.len() > 0 {
            field.node.attrs[0].span.lo()
        } else {
            field.span.lo()
        }
    };

    let unblock_me = |trivial| closure();

    let empty = |arg| {};

    let simple = |arg| {
        // comment formatting
        foo(arg)
    };

    let test = || {
        do_something();
        do_something_else();
    };

    let arg_test =
        |big_argument_name, test123| looooooooooooooooooong_function_naaaaaaaaaaaaaaaaame();

    let arg_test =
        |big_argument_name, test123| looooooooooooooooooong_function_naaaaaaaaaaaaaaaaame();

    let simple_closure = move || -> () {};

    let closure = |input: Ty| -> Option<String> { foo() };

    let closure_with_return_type =
        |aaaaaaaaaaaaaaaaaaaaaaarg1, aaaaaaaaaaaaaaaaaaaaaaarg2| -> Strong { "sup".to_owned() };

    |arg1, arg2, _, _, arg3, arg4| {
        let temp = arg4 + arg3;
        arg2 * arg1 - temp
    };
}
