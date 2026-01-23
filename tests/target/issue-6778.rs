#![allow(dead_code)]

struct Parameter {
    required: bool,
    description: &'static str,
}

pub struct Test;

impl Test {
    fn parameters(&self) -> &'static [Parameter] {
        &[
            Parameter {
                required: true,
                description: "Foo",
            },
            Parameter {
                required: false,
                description: "Bar
This string is exactly 100 chars long. Delete one character to make it 99 chars long and it'll work!",
            },
        ]
    }
}

fn main() {}
