// rustfmt-format_asm_macro: true

// collection of examples from The Rust Unstable Book found at
// https://doc.rust-lang.org/unstable-book/library-features/asm.html

// # Basic usage
// Let us start with the simplest possible example
asm!("nop");

// # Inputs and outputs
// This will write the value 5 into the u64 variable x
asm!("mov {}, 5", out(reg) x);

// This will add 5 to the input in variable i and write the result to variable o
asm!("mov {0}, {1}", "add {0}, {number}", out(reg) o, in(reg) i, number = const 5,);

// We can further refine the above example to avoid the mov instruction
// We can see that inout is used to specify an argument that is both input and output
asm!("add {0}, {number}", inout(reg) x, number = const 5);

// It is also possible to specify different variables for the input and output parts of an
// inout operand
asm!("add {0}, {number}", inout(reg) x => y, number = const 5);

// # Late output operands
// To guarantee optimal performance it is important to use as few registers as possible.
// To achieve this Rust provides a ``lateout`` specifier.
// This can be used on any output that is written only after all inputs have been consumed.
asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);

// Here is an example where inlateout cannot be used:
asm!("add {0}, {1}", "add {0}, {2}", inout(reg) a, in(reg) b, in(reg) c);

// # Explicit register operands
// Some instructions require that the operands be in a specific register
// Therefore, Rust inline assembly provides some more specific constraint specifiers.
asm!("out 0x64, eax", in("eax") cmd);

// Consider this example which uses the x86 mul instruction:
asm!(
// The x86 mul instruction takes rax as an implicit input and writes
// the 128-bit result of the multiplication to rax:rdx.
"mul {}",
in(reg) a,
inlateout("rax") b => lo,
lateout("rdx") hi
);

// # Clobbered registers
// In many cases inline assembly will modify state that is not needed as an output.
// This state is generally referred to as being "clobbered".
// We need to tell the compiler about this since it may need to save and restore this
// state around the inline assembly block.
// In the example below we use the cpuid instruction to get the L1 cache size.
// This instruction writes to eax, ebx, ecx, and edx, but for the cache size we only care
// about the contents of ebx and ecx. However we still need to tell the compiler that
// eax and edx have been modified so that it can save any values that were in these registers
// before the asm. This is done by declaring these as outputs but with _
asm!(
"cpuid",
// EAX 4 selects the "Deterministic Cache Parameters" CPUID leaf
inout("eax") 4 => _,
// ECX 0 selects the L0 cache information.
inout("ecx") 0 => ecx,
lateout("ebx") ebx,
lateout("edx") _,
);

// This can also be used with a general register class (e.g. reg) to obtain a scratch register
// for use inside the asm code:
asm!(
"mov {tmp}, {x}",
"shl {tmp}, 1",
"shl {x}, 2",
"add {x}, {tmp}",
x = inout(reg) x,
tmp = out(reg) _,
);

// # Symbol operands and ABI clobbers
// A special operand type, sym, allows you to use the symbol name of a fn or static in inline
// assembly code. Note that the fn or static item does not need to be public or #[no_mangle]:
// the compiler will automatically insert the appropriate mangled symbol name into the assembly code
extern "C" fn foo(arg: i32) -> i32 {
    println!("arg = {}", arg);
    arg * 2
}
asm!(
"call {}",
sym foo,
// 1st argument in rdi
in("rdi") arg,
// Return value in rax
out("rax") result,
// Mark all registers which are not preserved by the "C" calling
// convention as clobbered.
clobber_abi("C"),
);

// # Register template modifiers
// In some cases, fine control is needed over the way a register name is formatted when inserted
// into the template string. This is needed when an architecture's assembly language has several
// names for the same register, each typically being a "view" over a subset of the register
// (e.g. the low 32 bits of a 64-bit register).
//
// By default the compiler will always choose the name that refers to the full register size
// (e.g. rax on x86-64, eax on x86, etc).
//
// This default can be overriden by using modifiers on the template string operands, just like
// you would with format strings:
// In this example, we use the reg_abcd register class to restrict the register allocator to the
// 4 legacy x86 register (ax, bx, cx, dx)
asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);

// # Memory address operands
// Sometimes assembly instructions require operands passed via memory addresses/memory locations.
// You have to manually use the memory address syntax specified by the target architecture.
// For example, on x86/x86_64 using intel assembly syntax, you should wrap inputs/outputs in
// [] to indicate they are memory operands:
asm!("fldcw [{}]", in(reg) &control, options(nostack));

// # Labels
//  you should only use GNU assembler numeric local labels inside inline assembly code.
// https://sourceware.org/binutils/docs/as/Symbol-Names.html#Local-Labels
asm!(
    "mov {0}, 10",
    "2:",
    "sub {0}, 1",
    "cmp {0}, 3",
    "jle 2f",
    "jmp 2b",
    "2:",
    "add {0}, 2",
    out(reg) a
);

// # Options
// By default, an inline assembly block is treated the same way as an external FFI function call
// with a custom calling convention: it may read/write memory, have observable side effects, etc.
// However, in many cases it is desirable to give the compiler more information about what the
// assembly code is actually doing so that it can optimize better.
// Options can be provided as an optional final argument to the asm! macro.
asm!("add {0}, {1}", inlateout(reg) a, in(reg) b, options(pure, nomem, nostack));
