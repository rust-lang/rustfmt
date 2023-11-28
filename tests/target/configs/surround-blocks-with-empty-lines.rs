// rustfmt-surround_blocks_with_empty_lines: true

fn main() {
    // control flow at start of function
    if thing {
        println!("Hello, world!");
    }

    let x = 42;

    // two control flows in a row
    if lorem {
        println!("ipsum!");
    } else {
        println!("dolor!");
    }

    if other {
        println!("start");
    } else if middle {
        println!("middle!");
    } else {
        println!("end");
    }

    if end {
        println!("end");
    }

    let y = 24;

    // control flow at end of function
    match magi {
        Homura => "Akemi",
        Madoka => "Kaname",
    }
}

fn for_loop() {
    for _ in 0..10 {
        println!("Hello, world!");
    }
}

fn for_loop_double() {
    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }
}

fn for_loop_triple() {
    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }
}

fn for_loop_quad() {
    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }

    for _ in 0..10 {
        println!("Hello, world!");
    }
}

fn let_for_loop_block() {
    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };
}

fn let_for_loop_double() {
    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };
}

fn let_for_loop_triple() {
    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };
}

fn let_for_loop_quad() {
    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };

    let _ = for _ in 0..10 {
        println!("Hello, world!");
        break thing;
    };
}

fn while_loop() {
    while true {
        println!("Hello, world!");
    }
}

fn while_loop_double() {
    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }
}

fn while_loop_triple() {
    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }
}

fn while_loop_quad() {
    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }

    while true {
        println!("Hello, world!");
    }
}

fn let_while_loop_block() {
    let _ = while true {
        println!("Hello, world!");
        break thing;
    };
}

fn let_while_loop_double() {
    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };
}

fn let_while_loop_triple() {
    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };
}

fn let_while_loop_quad() {
    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };

    let _ = while true {
        println!("Hello, world!");
        break thing;
    };
}

fn if_block() {
    if true {
        println!("Hello, world!");
    }
}

fn if_double() {
    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }
}

fn if_triple() {
    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }
}

fn if_quad() {
    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    }
}

fn let_if_block() {
    let _ = if true {
        println!("Hello, world!");
    };
}

fn let_if_double() {
    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };
}

fn let_if_triple() {
    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };
}

fn let_if_quad() {
    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    };
}

fn if_else_block() {
    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_double() {
    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_triple() {
    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_quad() {
    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn let_if_else_block() {
    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_double() {
    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_triple() {
    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_quad() {
    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn else_if_block() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }
}

fn else_if_double() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }
}

fn else_if_triple() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }
}

fn else_if_quad() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    }
}

fn let_if_else_if_block() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_double() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_triple() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_quad() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    };
}

fn if_else_if_else_block() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_if_else_double() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_if_else_triple() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn if_else_if_else_quad() {
    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }

    if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    }
}

fn let_if_else_if_else_block() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_double() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_triple() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn let_if_else_if_else_quad() {
    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };

    let _ = if true {
        println!("Hello, world!");
    } else if false {
        println!("Hello, world!");
    } else {
        println!("Hello, world!");
    };
}

fn loop_block() {
    loop {
        println!("Hello, world!");
        break;
    }
}

fn loop_double() {
    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }
}

fn loop_triple() {
    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }
}

fn for_loop_quad() {
    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }

    loop {
        println!("Hello, world!");
        break;
    }
}

fn let_loop_block() {
    let _ = loop {
        println!("Hello, world!");
        break;
    };
}

fn let_loop_double() {
    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };
}

fn let_loop_triple() {
    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };
}

fn let_loop_quad() {
    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };

    let _ = loop {
        println!("Hello, world!");
        break;
    };
}

fn match_block() {
    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }
}

fn match_double() {
    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }
}

fn match_triple() {
    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }
}

fn match_quad() {
    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }

    match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    }
}

fn let_match_block() {
    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };
}

fn let_match_double() {
    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };
}

fn let_match_triple() {
    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };
}

fn let_match_quad() {
    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };

    let _ = match true {
        true => println!("Hello, world!"),
        false => println!("Hello, world!"),
    };
}

fn let_var_with_block() {
    let _ = {
        println!("Hello, world!");
    };
}

fn let_var_with_double() {
    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };
}

fn let_var_with_triple() {
    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };
}

fn let_var_with_quad() {
    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };

    let _ = {
        println!("Hello, world!");
    };
}

fn nested() {
    if true {
        if true {
            if true {
                println!("Hello, world!");
            }
        }
    }
}
