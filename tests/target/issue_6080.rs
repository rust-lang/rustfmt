// rustfmt-struct_field_align_threshold: 30

#[derive(Default)]
struct Foo {
    id:   u8,
    age:  u8,
    name: String,
}

fn main() {
    foo = Foo {
        id:   5,
        name: "John".into(),
        ..Default::default()
    };
}
