// rustfmt-format_asm_macro: true

asm!(
    "mov {}, 5",
    out(reg) x
);

asm!(
    "mov {}, 5",
    out("eax") x
);

asm![
    "mov {}, 5",
    out(reg) x
];

asm![
    "mov {}, 5",
    out("eax") x
];

asm! {
    "mov {}, 5",
    out(reg) x
}

asm! {
    "mov {}, 5",
    out("eax") x
}
