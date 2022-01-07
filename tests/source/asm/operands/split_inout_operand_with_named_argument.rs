// rustfmt-format_asm_macro: true
// rustfmt-max_width: 50

// case 1: Don't break; Named argument, operand, in_expr, and out_expr all fit on the same line
asm!("instruction {}", name = inout(reg) in_expr => out_expr,);
asm!("instruction {}", name = inlateout(reg) in_expr => out_expr,);
asm!("instruction {}", name = inout(reg) in_expr => _,);
asm!("instruction {}", name = inlateout(reg) in_expr => _,);

// case 2: Break after assignment; Operand, in_expr, and out_expr all fit on the same line
asm!("instruction {}", some_long_name = inout(reg) in_expr => out_expr,);
asm!("instruction {}", some_long_name = inlateout(reg) in_expr => out_expr,);
asm!("instruction {}", some_very_long_name = inout(reg_abcd) in_expr => _,);
asm!("instruction {}", some_very_long_name = inlateout(reg_abcd) in_expr => _,);

// case 3: Break after assignment; Also Break before =>
//     in_expr: (identifier)
//     out_expr: (identifier)
asm!("instruction {}", some_long_name = inout(reg) very_long_expression => very_long_out_expression,);
asm!("instruction {}", some_long_name = inlateout(reg) very_long_expression => very_long_out_expression,);

// case 4: Break after assignment; Also Break before =>
//     in_expr: (chain)
//     out_expr: (identifier)
asm!("instruction {}", some_long_name = inout(reg) function_name().method_name(method_arg).further_chained_method() => very_long_out_expression,);
asm!("instruction {}", some_long_name = inlateout(reg) function_name().method_name(method_arg).further_chained_method() => very_long_out_expression,);

// case 5: Break after assignment; Also Break before =>
//     in_expr: (function)
//     out_expr: (identifier)
asm!("instruction {}", some_long_name = inout(reg) long_function_name(long_function_argument_expression) => very_long_out_expression,);
asm!("instruction {}", some_long_name = inlateout(reg) long_function_name(long_function_argument_expression) => very_long_out_expression,);

// case 6: Break after assignment; Also Break before =>
//     in_expr: (identifier)
//     out_expr: (chain)
asm!("instruction {}", some_long_name = inout(reg) very_long_expression => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", some_long_name = inlateout(reg) very_long_expression => function_name().method_name(method_arg).further_chained_method(),);

// case 7: Break after assignment; Also Break before =>
//     in_expr: (chain)
//     out_expr: (chain)
asm!("instruction {}", some_long_name = inout(reg) function_name().method_name(method_arg).further_chained_method() => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", some_long_name = inlateout(reg) function_name().method_name(method_arg).further_chained_method() => function_name().method_name(method_arg).further_chained_method(),);

// case 8: Break after assignment; Also Break before =>
//     in_expr: (function)
//     out_expr: (chain)
asm!("instruction {}", some_long_name = inout(reg) long_function_name(long_function_argument_expression) => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", some_long_name = inlateout(reg) long_function_name(long_function_argument_expression) => function_name().method_name(method_arg).further_chained_method(),);

// case 9: Break after assignment; Also Break before =>
//     in_expr: (identifier)
//     out_expr: (function)
asm!("instruction {}", some_long_name = inout(reg) very_long_expression => long_function_name(long_function_argument_expression),);
asm!("instruction {}", some_long_name = inlateout(reg) very_long_expression => long_function_name(long_function_argument_expression),);

// case 10: Break after assignment; Also Break before =>
//     in_expr: (chain)
//     out_expr: (function)
asm!("instruction {}", some_long_name = inout(reg) function_name().method_name(method_arg).further_chained_method() => long_function_name(long_function_argument_expression),);
asm!("instruction {}", some_long_name = inlateout(reg) function_name().method_name(method_arg).further_chained_method() => long_function_name(long_function_argument_expression),);

// case 11: Break after assignment; Also Break before =>
//     in_expr: (function)
//     out_expr: (function)
asm!("instruction {}", some_long_name = inout(reg) long_function_name(long_function_argument_expression) => long_function_name(long_function_argument_expression),);
asm!("instruction {}", some_long_name = inlateout(reg) long_function_name(long_function_argument_expression) => long_function_name(long_function_argument_expression),);

// case 12: Break after assignment; Also Break before =>
//     in_expr: (identifier)
//     out_expr: _
asm!("instruction {}", some_long_name = inout(reg) very_very_very_long_expression => _,);
asm!("instruction {}", some_long_name = inlateout(reg) very_very_very_long_expression => _,);

// case 13: Break after assignment; Also Break before =>
//     in_expr: (chain)
//     out_expr: _
asm!("instruction {}", some_long_name = inout(reg) function_name().method_name(method_arg).further_chained_method() => _,);
asm!("instruction {}", some_long_name = inlateout(reg) function_name().method_name(method_arg).further_chained_method() => _,);

// case 14: Break after assignment; Also Break before =>
//     in_expr: (function)
//     out_expr: _
asm!("instruction {}", some_long_name = inout(reg) long_function_name(long_function_argument_expression) => _,);
asm!("instruction {}", some_long_name = inlateout(reg) long_function_name(long_function_argument_expression) => _,);
