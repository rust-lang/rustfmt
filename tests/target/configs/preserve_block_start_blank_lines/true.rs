// rustfmt-preserve_block_start_blank_lines: true

fn noop() {
    println!("hi");
}

fn say_hi() {


    println!("hi");
}

fn say_hi() {


    #![attr]
    println!("hi");
}

fn say_hi() {


    //! Abcdef

    println!("hi");
}

fn say_hi() {


    // Abcdef

    println!("hi");
}

fn say_hi() {
    // Abcdef

    println!("hi");
}

fn container() {
    if true {


        // comment
        do_work();
    } else {


        // comment
        other_work();
    }

    let val = {


        // comment
        get_val();
    };
}

trait Hello {


    // comment

    fn say_hi() -> &str;
}

trait Hello {
    // comment

    fn say_hi() -> &str;
}

trait Hello {



    fn say_hi() -> &str;
}

trait Hello {
    fn say_hi() -> &str;
}

impl Hello for &str {


    // comment

    fn say_hi() -> &str {
        "hi"
    }
}

impl Hello for &str {
    // comment

    fn say_hi() -> &str {
        "hi"
    }
}

impl Hello for &str {



    fn say_hi() -> &str {
        "hi"
    }
}

impl Hello for &str {
    fn say_hi() -> &str {
        "hi"
    }
}

mod Hi {


    // comment

    fn say_hi() -> &str;
}

mod Hi {


    /// comment

    fn say_hi() -> &str;
}

mod Hi {


    // comment

    /// comment

    fn say_hi() -> &str;
}

mod Hi {


    // comment

    #![attr]

    fn say_hi() -> &str;
}

mod Hi {


    #![attr]

    fn say_hi() -> &str;
}

mod Hi {
    // comment
    fn say_hi() -> &str;
}

mod Hi {
    fn say_hi() -> &str;
}

extern "C" {


    // comment

    fn say_hi() -> &str;
}

extern "C" {


    /// comment
    fn say_hi() -> &str;
}

extern "C" {


    // comment

    /// comment
    fn say_hi() -> &str;
}

extern "C" {


    // comment

    #[attr]
    fn say_hi() -> &str;
}

extern "C" {
    // comment
    fn say_hi() -> &str;
}

extern "C" {
    fn say_hi() -> &str;
}
