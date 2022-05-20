// rustfmt-format_asm_macro: false

asm!("add {0}, {number}", inout(reg) x, number = const 5);
