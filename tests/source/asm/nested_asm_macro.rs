// rustfmt-format_asm_macro: true

#![feature(asm)]
extern "C" fn foo(arg: i32) -> i32 {
    println!("arg = {}", arg);
    arg * 2
}

fn call_with_parens(arg: i32) -> i32 {
    unsafe {
        let result;
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
        result
    }
}

fn call_with_brackets(arg: i32) -> i32 {
    unsafe {
        let result;
        asm![
        "call {}",
        sym foo,
        // 1st argument in rdi
        in("rdi") arg,
        // Return value in rax
        out("rax") result,
        // Mark all registers which are not preserved by the "C" calling
        // convention as clobbered.
        clobber_abi("C"),
        ];
        result
    }
}

fn call_with_braces(arg: i32) -> i32 {
    unsafe {
        let result;
        asm! {
        "call {}",
        sym foo,
        // 1st argument in rdi
        in("rdi") arg,
        // Return value in rax
        out("rax") result,
        // Mark all registers which are not preserved by the "C" calling
        // convention as clobbered.
        clobber_abi("C"),
        }
        result
    }
}
