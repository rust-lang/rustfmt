// rustfmt-format_asm_macro: true
// rustfmt-max_width: 50

// case 1: Don't break; named argument operand, and expression fit on the same line
asm!(
    "instruction {}",
    name = in(reg) expression,
);
asm!(
    "instruction {}",
    name = out(reg) expression,
);
asm!(
    "instruction {}",
    name = lateout(reg) expression,
);
asm!(
    "instruction {}",
    name = inlateout(reg) expression,
);
asm!(
    "instruction {}",
    name = const expression,
);
asm!(
    "instruction {}",
    name = sym expression,
);
asm!(
    "instruction {}",
    name = out(reg) _,
);
asm!(
    "instruction {}",
    name = lateout(reg) _,
);

// case 2: Break after assignment; Operand and expression on the same line
asm!(
    "instruction {}",
    some_long_name =
        in(reg) unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg) unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg) unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg) unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        const unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        sym unbreakable_expression,
);

// case 3: Break after assignment: Operand and expression on different lines (except const and sym)
asm!(
    "instruction {}",
    some_long_name =
        in(reg_abcd)
            extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg_abcd)
            extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg_abcd)
            extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg_abcd)
            extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        const extremely_long_unbreakable_expression,
);
asm!(
    "instruction {}",
    some_long_name =
        sym extremely_long_unbreakable_expression,
);

// case 4 Don't Break after assignemt; Named argument operand and start of chain on the same line (except const)
asm!(
    "instruction {}",
    name = in(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    name = out(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    name = lateout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    name = inout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    name = inlateout(reg) function_name()
        .method_name(method_arg)
        .further_chained_method(),
);
asm!(
    "instruction {}",
    name = const function_name()
        .method_name(method_arg)
        .further_chained_method(),
);

// case 5: Break after assignment; Operand and start of chain on the same line
asm!(
    "instruction {}",
    some_long_name =
        in(reg_abcd) long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg_abcd) long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg_abcd) long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        inout(reg_abcd) long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg_abcd) long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        const long_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);

// case 6: Break after assignment; Operand and chain on different lines (except const)
asm!(
    "instruction {}",
    some_long_name =
        in(reg_abcd)
            extremely_long_unbreakable_function_name()
                .method_name(method_arg)
                .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg_abcd)
            extremely_long_unbreakable_function_name()
                .method_name(method_arg)
                .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg_abcd)
            extremely_long_unbreakable_function_name()
                .method_name(method_arg)
                .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        inout(reg_abcd)
            extremely_long_unbreakable_function_name()
                .method_name(method_arg)
                .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg_abcd)
            extremely_long_unbreakable_function_name()
                .method_name(method_arg)
                .further_chained_method(),
);
asm!(
    "instruction {}",
    some_long_name =
        const extremely_long_unbreakable_function_name()
            .method_name(method_arg)
            .further_chained_method(),
);

// case 7 Don't Break after assignemt; Named argument operand and start of function on the same line
asm!(
    "instruction {}",
    name = in(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    name = out(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    name = lateout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    name = inout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    name = inlateout(reg) long_function_name(
        long_function_argument_expression
    ),
);
asm!(
    "instruction {}",
    name = const long_function_name(
        long_function_argument_expression
    ),
);

// case 8 Break after assignment; Operand and start of function on the same line
asm!(
    "instruction {}",
    some_long_name =
        in(reg) long_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg) long_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg) long_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    some_long_name =
        inout(reg) long_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg) long_function_name(
            long_function_argument_expression
        ),
);
asm!(
    "instruction {}",
    some_long_name =
        const long_function_name(
            long_function_argument_expression
        ),
);

// case 9: Break after assignment; Operand and function on different lines (except const)
asm!(
    "instruction {}",
    some_long_name =
        in(reg)
            extremely_long_unbreakable_function_name(
                long_function_argument_expression
            ),
);
asm!(
    "instruction {}",
    some_long_name =
        out(reg)
            extremely_long_unbreakable_function_name(
                long_function_argument_expression
            ),
);
asm!(
    "instruction {}",
    some_long_name =
        lateout(reg)
            extremely_long_unbreakable_function_name(
                long_function_argument_expression
            ),
);
asm!(
    "instruction {}",
    some_long_name =
        inout(reg)
            extremely_long_unbreakable_function_name(
                long_function_argument_expression
            ),
);
asm!(
    "instruction {}",
    some_long_name =
        inlateout(reg)
            extremely_long_unbreakable_function_name(
                long_function_argument_expression
            ),
);
asm!(
    "instruction {}",
    some_long_name =
        const extremely_long_unbreakable_function_name(
            long_function_argument_expression
        ),
);
