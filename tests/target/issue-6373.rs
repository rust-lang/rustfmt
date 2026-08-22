fn main() {
    let x: Option<i32> = Some(10);
    let value = match x {
        Some(i) => i, /* comment */
        None => 0,
    };
    println!("{}", value);
}
