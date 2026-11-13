//@ rustfmt-force_explicit_return: true
// Force explicit return statements

fn add_one(x: u32) -> u32 {
    x + 1
}

fn foo() {
    println!("hello");
}

fn bar() -> i32 {
    if true {
        1
    } else {
        2
    }
}

fn baz() -> Result<i32, &str> {
    Ok(42)
}

fn empty_body() {}

fn unit_return() -> () {
    let x = 1;
    println!("{}", x);
}

fn early_return(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    -x
}

fn diverging() -> ! {
    panic!("never returns")
}

fn explicit_return(x: i32) -> i32 {
    return x * 2;
}

fn closure_example() {
    let c = || 42;
    c();
}

async fn async_fn() -> i32 {
    42
}

const fn const_fn() -> i32 {
    42
}

unsafe fn unsafe_fn() -> i32 {
    42
}

extern "C" fn extern_fn() -> i32 {
    42
}

// Single-line functions
fn single_line_expr() -> i32 { 42 }
fn single_line_unit() { println!("hi"); }
fn single_line_empty() {}
fn single_line_never() -> ! { panic!() }