fn less_than_or_equal_operand() {
    let x: u32 = 100;
    if x as i32<> <= 0 {
        // ...
    }
}

fn long_binary_op_chain_no_wrap() {
    let x: u32 = 100;
    if x as i32 <= 0 && x as i32 <= 0 && x as i32 <= 0 && x as i32 <= 0 && x as i32 <= 0 && x as i32 <= 0 && x as i32 <= 0 {
        // ...
    }
}
