// rustfmt-style_edition: 2024

fn main() {
    some_struct.foo(format!(
        "{}", if true {
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    } else {
            ""
        },
    ));
}
