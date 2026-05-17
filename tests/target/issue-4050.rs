// https://github.com/rust-lang/rustfmt/issues/4050
// Closures with block body incorrectly formatted when followed by call and method chain.

fn main() {
    // Simple block with call and method chain
    let a = || { 42 }().to_string();

    // Multi-statement block with call and method chain
    let b = || {
        println!();
        || 0
    }()
    .to_string();

    // Block without call, with method chain
    let c = || { 42 }.to_string();

    // Nested closure block with call and method chain
    let d = || {
        println!("hello");
        42
    }()
    .to_string();
}
