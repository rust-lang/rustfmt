// rustfmt-format_asm_macro: true
// rustfmt-max_width: 30

// Options can be provided as an optional final argument to the asm! macro

// with trailing commas
asm!("some asm template", options(nostack),);

asm!("some asm template", options(pure, nomem, nostack),);

asm!("some asm template", options(pure, nomem, nostack, preserves_flags),);

// without trailing commas
asm!("some asm template", options(nostack));

asm!("some asm template", options(pure, nomem, nostack));

asm!("some asm template", options(pure, nomem, nostack, preserves_flags));

// with trailing commas
asm!["some asm template", options(nostack),];

asm!["some asm template", options(pure, nomem, nostack),];

asm!["some asm template", options(pure, nomem, nostack, preserves_flags),];

// without trailing commas
asm!["some asm template", options(nostack)];

asm!["some asm template", options(pure, nomem, nostack)];

asm!["some asm template", options(pure, nomem, nostack, preserves_flags)];

// with trailing commas
asm! {"some asm template", options(nostack),}

asm! {"some asm template", options(pure, nomem, nostack),}

asm! {"some asm template", options(pure, nomem, nostack, preserves_flags),}

// without trailing commas
asm! {"some asm template", options(nostack)}

asm! {"some asm template", options(pure, nomem, nostack)}

asm! {"some asm template", options(pure, nomem, nostack, preserves_flags)}
