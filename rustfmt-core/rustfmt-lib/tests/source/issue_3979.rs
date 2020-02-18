// https://github.com/rust-lang/rustfmt/issues/3979

fn main() {
    let o_num = Some(123);
    
    let x = if let Some(n) =
        // this is a test comment
        // to see if rustfmt will break
        // after using the lateset version
        o_num {
            n * 2
        } else {
            0
        };
    
    println!("Number: {}", x);
}