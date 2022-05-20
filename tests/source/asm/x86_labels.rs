// rustfmt-format_asm_macro: true

// x86 Assembly Language Reference Manual:
// https://docs.oracle.com/cd/E19120-01/open.solaris/817-5477/esqaq/index.html

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!("label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4");

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm!(".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4");

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!("0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4");

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!("label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4",);

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm!(".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4",);

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!("0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4",);

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!["label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4"];

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm![".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4"];

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!["0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4"];

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!["label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4",];

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm![".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4",];

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!["0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4",];

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!{"label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4"}

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm!{".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4"}

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!{"0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4"}

// A symbolic label consists of an identifier (or symbol) followed by a colon (:)
asm!{"label1:", "instruction1", "instruction2", "label2:", "instruction3", "instruction4",}

// Symbolic labels with identifiers beginning with a period (.) (ASCII 0x2E) are
// considered to have local scope and are not included in the object file's symbol table
asm!{".lable1", "instruction1", "instruction2", ".label2", "instruction3", "instruction4",}

// A numeric label consists of a single digit in the range zero (0) through nine (9)
// followed by a colon (:)
asm!{"0:", "instruction1", "instruction2", "9:", "instruction3", "instruction4",}