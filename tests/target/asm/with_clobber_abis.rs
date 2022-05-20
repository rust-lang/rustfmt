// rustfmt-format_asm_macro: true

// The clobber_abi argument to asm! tells the compiler to automatically insert the
// necessary clobber operands according to the given calling convention ABI
asm!(
    "some asm template",
    clobber_abi("C")
);

asm!(
    "some asm template",
    clobber_abi("C"),
);

asm![
    "some asm template",
    clobber_abi("C")
];

asm![
    "some asm template",
    clobber_abi("C"),
];

asm! {
    "some asm template",
    clobber_abi("C")
}

asm! {
    "some asm template",
    clobber_abi("C"),
}
