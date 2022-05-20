// rustfmt-format_asm_macro: true
// rustfmt-max_width: 31

asm!("nop");

asm!("can't fit it on one line");

asm!("nop",);

asm!("can't fit it on one line",);

asm!["nop"];

asm!["can't fit it on one line"];

asm!["nop",];

asm!["can't fit it on one line",];

asm! {"nop"}

asm! {"can't fit it on one line"}

asm! {"nop",}

asm! {"can't fit it on one line",}
