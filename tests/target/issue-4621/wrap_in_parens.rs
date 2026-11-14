fn less_than_operand() {
    let x: u32 = 100;
    if (x as i32) < 0 {
        // ...
    }
}

fn left_shift_operand() {
    let x: u32 = 100;
    if (x as i32) << 1 < 0 {
        // ...
    }
}

fn long_binary_op_chain_wrap_all() {
    let x: u32 = 100;
    if (x as i32) < 0
        && (x as i32) < 0
        && (x as i32) << 1 < 0
        && (x as i32) << 1 < 0
        && (x as i32) << 1 < 0
        && (x as i32) << 1 < 0
    {
        // ...
    }
}

fn long_binary_op_chain_wrap_some() {
    let x: u32 = 100;
    if (x as i32) < 0
        && x as i32 <= 0
        && (x as i32) << 1 < 0
        && x as i32 <= 0
        && (x as i32) << 1 < 0
    {
        // ...
    }
}
