//@ rustfmt-force_explicit_return: false
// Force explicit return statements (disabled)

fn add_one(x: u32) -> u32 {
    return x + 1;
}

fn foo() {
    println!("hello");
    return;
}

fn bar() -> i32 {
    return if true { 1 } else { 2 };
}

fn baz() -> Result<i32, &str> {
    return Ok(42);
}

fn empty_body() {
    return;
}

fn unit_return() -> () {
    let x = 1;
    println!("{}", x);
    return;
}

fn early_return(x: i32) -> i32 {
    if x > 0 {
        return x;
    }
    return -x;
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
    return;
}

async fn async_fn() -> i32 {
    return 42;
}

const fn const_fn() -> i32 {
    return 42;
}

unsafe fn unsafe_fn() -> i32 {
    return 42;
}

extern "C" fn extern_fn() -> i32 {
    return 42;
}

// Single-line functions
fn single_line_expr() -> i32 { return 42; }
fn single_line_unit() { println!("hi"); return; }
fn single_line_empty() { return; }