fn main() {
    let x = if true {

        1
          // In if
    } else {
        0
        // in else
    };

    for i in 0..2 {
        println!("Something");
        // in for
    }

    extern "C" {
        fn first();

        // TODO: add rest
    }
}
