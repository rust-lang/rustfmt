unsafe fn foo() -> i32 {
    42
}

fn main() {
    'label: loop {
        break 'label unsafe { foo() };
    };
}
