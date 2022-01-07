// rustfmt-format_asm_macro: true
// rustfmt-max_width: 50

// case 1: Operand, in_expr, and out_expr all fit on the same line
asm!("instruction {}", inout(reg) in_expr => out_expr,);
asm!("instruction {}", inlateout(reg) in_expr => out_expr,);
asm!("instruction {}", inout(reg) in_expr => _,);
asm!("instruction {}", inlateout(reg) in_expr => _,);

// case 2: Break before =>
//     in_expr: (identifier)
//     out_expr: (identifier)
asm!("instruction {}", inout(reg) very_long_expression => very_long_out_expression,);
asm!("instruction {}", inlateout(reg) very_long_expression => very_long_out_expression,);

// case 3: Break before =>
//     in_expr: (chain)
//     out_expr: (identifier)
asm!("instruction {}", inout(reg) function_name().method_name(method_arg).further_chained_method() => very_long_out_expression,);
asm!("instruction {}", inlateout(reg) function_name().method_name(method_arg).further_chained_method() => very_long_out_expression,);

// case 4: Break before =>
//     in_expr: (function)
//     out_expr: (identifier)
asm!("instruction {}", inout(reg) long_function_name(long_function_argument_expression) => very_long_out_expression,);
asm!("instruction {}", inlateout(reg) long_function_name(long_function_argument_expression) => very_long_out_expression,);

// case 5: Break before =>
//     in_expr: (identifier)
//     out_expr: (chain)
asm!("instruction {}", inout(reg) very_long_expression => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", inlateout(reg) very_long_expression => function_name().method_name(method_arg).further_chained_method(),);

// case 6: Break before =>
//     in_expr: (chain)
//     out_expr: (chain)
asm!("instruction {}", inout(reg) function_name().method_name(method_arg).further_chained_method() => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", inlateout(reg) function_name().method_name(method_arg).further_chained_method() => function_name().method_name(method_arg).further_chained_method(),);

// case 7: Break before =>
//     in_expr: (function)
//     out_expr: (chain)
asm!("instruction {}", inout(reg) long_function_name(long_function_argument_expression) => function_name().method_name(method_arg).further_chained_method(),);
asm!("instruction {}", inlateout(reg) long_function_name(long_function_argument_expression) => function_name().method_name(method_arg).further_chained_method(),);

// case 8: Break before =>
//     in_expr: (identifier)
//     out_expr: (function)
asm!("instruction {}", inout(reg) very_long_expression => long_function_name(long_function_argument_expression),);
asm!("instruction {}", inlateout(reg) very_long_expression => long_function_name(long_function_argument_expression),);

// case 9: Break before =>
//     in_expr: (chain)
//     out_expr: (function)
asm!("instruction {}", inout(reg) function_name().method_name(method_arg).further_chained_method() => long_function_name(long_function_argument_expression),);
asm!("instruction {}", inlateout(reg) function_name().method_name(method_arg).further_chained_method() => long_function_name(long_function_argument_expression),);

// case 10: Break before =>
//     in_expr: (function)
//     out_expr: (function)
asm!("instruction {}", inout(reg) long_function_name(long_function_argument_expression) => long_function_name(long_function_argument_expression),);
asm!("instruction {}", inlateout(reg) long_function_name(long_function_argument_expression) => long_function_name(long_function_argument_expression),);

// case 11: Break before =>
//     in_expr: (identifier)
//     out_expr: _
asm!("instruction {}", inout(reg) very_very_very_long_expression => _,);
asm!("instruction {}", inlateout(reg) very_very_very_long_expression => _,);

// case 12: Break before =>
//     in_expr: (chain)
//     out_expr: _
asm!("instruction {}", inout(reg) function_name().method_name(method_arg).further_chained_method() => _,);
asm!("instruction {}", inlateout(reg) function_name().method_name(method_arg).further_chained_method() => _,);

// case 13: Break before =>
//     in_expr: (function)
//     out_expr: _
asm!("instruction {}", inout(reg) long_function_name(long_function_argument_expression) => _,);
asm!("instruction {}", inlateout(reg) long_function_name(long_function_argument_expression) => _,);
