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
