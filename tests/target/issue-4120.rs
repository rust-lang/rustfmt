fn main() {
    let x = if true {
        1
        // In if
    } else {
        0
        // In else
    };

    let x = if true {
        1
        /* In if */
    } else {
        0
        /* In else */
    };

    for i in 0..2 {
        println!("Something");
        // In for
    }

    for i in 0..2 {
        println!("Something");
        /* In for */
    }

    extern "C" {
        fn first();

        // In foreign mod
    }

    extern "C" {
        fn first();

        /* In foreign mod */
    }
}
