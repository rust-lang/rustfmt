// rustfmt-trailing_semicolon: false

fn main() {
    println!("{}", greet());
}

fn greet() -> String {
    return "Hello, b!".to_string();
}

fn foo() {}
fn main() {
    return;
    foo()
}
