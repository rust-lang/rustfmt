// rustfmt-style_edition: 2027

fn main() {
    some_struct.foo(format!(
        "{}",
        if true {
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        } else {
            ""
        },
    ));
}

fn with_comment() {
    some_struct.foo(format!(
        "{}",
        if true {
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" // comment
        } else {
            ""
        },
    ));
}

fn code_overflow() {
    some_struct.foo(format!(
        "{}", if true {
            identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_
                    } else {
            ""
        },
    ));
}

fn code_overflow_with_comment() {
    some_struct.foo(format!(
        "{}", if true {
            identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_identifier_ // comment
                    } else {
            ""
        },
    ));
}

fn short_receiver() {
    a.foo(format!(
        "{}",
        if true {
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        } else {
            ""
        },
    ));
}
