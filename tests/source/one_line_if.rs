fn plain_if(x: bool) -> u8 {
    if x {
        0
    } else {
        1
    }
}

fn paren_if(x: bool) -> u8 {
    (if x { 0 } else { 1 })
}

fn let_if(x: bool) -> u8 {
    let x = if x { 0 } else { 1 };
    x
}

fn return_if(x: bool) -> u8 {
    return if x {
        0
    } else {
        1
    };
}
