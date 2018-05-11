// rustfmt-match_reference_style: dereference
// Normalize match reference style

fn hello(name: &Option<&str>) {
    match *name {
        Some(name) => println!("Hello {}!", name),
        None => println!("I don't know who you are."),
    }
}

fn kiss(name: &Option<&str>) {
    match *name {
        Some(name) => println!("Kiss {}!", name),
        None => println!("I don't know who you are."),
    }
}

fn main() {
    let name = Some("rustfmt");

    hello(&name);
    kiss(&name);
}
