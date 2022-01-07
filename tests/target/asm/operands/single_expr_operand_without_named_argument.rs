// rustfmt-format_asm_macro: true
// rustfmt-max_width: 50

// case 1: Don't break; Only out or lateout operands can set the out expression to '_'
asm!(
    "instruction {}",
    out(reg) _,
);
asm!(
    "instruction {}",
    lateout(reg) _,
);

// case 2: Operand and expression on the same line
asm!(
    "instruction {}",
    in(reg) long_unbreakable_expression,
);
asm!(
    "instruction {}",
    out(reg) long_unbreakable_expression,
);
asm!(
    "instruction {}",
    lateout(reg) long_unbreakable_expression,
);
asm!(
    "instruction {}",
    inlateout(reg)
        long_unbreakable_expression,
);
asm!(
    "instruction {}",
    const long_unbreakable_expression,
);
asm!(
    "instruction {}",
    sym long_unbreakable_expression,
);

// case 3: Break before expression (except const and sym)
asm!(
    "instruction {}",
    in(reg_abcd)
        extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    out(reg_abcd)
        extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    lateout(reg_abcd)
        extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    inlateout(reg_abcd)
        extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    const extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    sym extremely_long_unbreakable_expression,
);

// case 4: Operand and start of chain on the same line
asm!(
    "instruction {}",
    in(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    out(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    lateout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    inout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    inlateout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    const function_name()
        .method_name(method_arg)
        .further_chained_method(),
);

// case 5: Break before chain (except const)
asm!(
    "instruction {}",
    in(reg_abcd)
        extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    out(reg_abcd)
        extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    lateout(reg_abcd)
        extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    inout(reg_abcd)
        extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    inlateout(reg_abcd)
        extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    const extremely_long_unbreakable_function_name()
        .method_name(method_arg)
        .further_chained_method(),
);

// case 6: Operand and start of function on the same line
asm!(
    "instruction {}",
    in(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    out(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    lateout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    inout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    inlateout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    const long_function_name(
        long_function_argument_expression
    ),
);

// case 7: Break before function (except const)
asm!(
    "instruction {}",
    in(reg)
        extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    out(reg)
        extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    lateout(reg)
        extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    inout(reg)
        extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    inlateout(reg)
        extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    const extremely_long_unbreakable_function_name(
        long_function_argument_expression
    ),
);
