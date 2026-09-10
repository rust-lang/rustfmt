// rustfmt-inline_attribute_width: 50
// rustfmt-edition: 2021

#[cfg(feature = "alloc")] use core::slice;

#[cfg(feature = "alloc")] use total_len_is::_50__;

#[cfg(feature = "alloc")]
use total_len_is::_51___;

#[cfg(feature = "alloc")]
// c
use d;

#[cfg(feature = "alloc")]
// comment
use total_len_is::_50__;

#[cfg(feature = "alloc")]
// comment
use total_len_is::_51___;

fn foo() {
    #[cfg(feature = "alloc")] use total_len::_50_;

    #[cfg(feature = "alloc")]
    use total_len::_51__;
}

#[cfg(feature = "alloc")] extern crate len_is_50_;

#[cfg(feature = "alloc")]
extern crate len_is_51__;

// https://github.com/rust-lang/rustfmt/issues/3343#issuecomment-589945611
extern "C" {
    #[no_mangle] fn foo();
}

extern "C" {
    #[no_mangle] fn total_len_is_49___________();
}

extern "C" {
    #[no_mangle] fn total_len_is_50____________();
}

extern "C" {
    #[no_mangle]
    fn total_len_is_51_____________();
}

fn main() {
    #[cfg(feature = "alloc")] ["total_len_is_50"];
    #[cfg(feature = "alloc")]
    ["total_len_is_51_"];
    #[cfg(feature = "alloc")] total_len_is_50__();
    #[cfg(feature = "alloc")]
    total_len_is_51___();
    #[cfg(feature = "alloc")]
    total_len_is_52____();
    #[cfg(feature = "alloc")]
    {
        foo();
    }
    {
        #[cfg(feature = "alloc")] foo();
    }
}

// https://github.com/rust-lang/rustfmt/pull/5538#issuecomment-1272367684
struct EventConfigWidget {
    #[widget] menu_delay: Spinner<u32>,
}

struct foo {
    #[x]
    #[y]
    z: bool,
}

struct foo {
    #[widget] len_is_50____________________: bool,
}

struct foo {
    #[widget]
    len_is_51_____________________: bool,
}

/// this is a comment to test is_sugared_doc property
use core::convert;

#[fooooo]
#[barrrrr]
use total_len_is_::_51______;

#[cfg(not(all(
    feature = "std",
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "haiku",
        target_os = "emscripten",
        target_os = "solaris",
        target_os = "cloudabi",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "fuchsia",
        windows,
        all(target_arch = "wasm32", feature = "stdweb"),
        all(target_arch = "wasm32", feature = "wasm-bindgen"),
    )
)))]
use core::slice;

fn foo() {
    // Literal expression
    #[cfg(feature = "len_is_50_____________")] 42;
    #[cfg(feature = "len_is_51______________")]
    42;

    // Path expression
    #[cfg(feature = "len_is_50__")] some_variable;
    #[cfg(feature = "len_is_51___")]
    some_variable;

    // Block expression
    #[cfg(feature = "len_is_50_______________")]
    {
        let x = 2;
    };
    #[cfg(feature = "len_is_51________________")]
    {
        let x = 2;
    };

    // Operator expression
    #[cfg(feature = "len_is_50__________")]
    1 + 2;
    #[cfg(feature = "len_is_51___________")]
    1 + 2;

    // Tuple expression
    #[cfg(feature = "len_is_50")] (1, "two", 3.0);
    #[cfg(feature = "len_is_51_")]
    (1, "two", 3.0);

    // Array expression
    #[cfg(feature = "len_is_50______")] [1, 2, 3];
    #[cfg(feature = "len_is_51_______")]
    [1, 2, 3];

    // Struct expression
    #[cfg(feature = "len_is_50_____")] S { f: 1 };
    #[cfg(feature = "len_is_51______")]
    S { f: 1 };

    #[cfg(feature = "len_is_50______")]
    MyStruct {
        field1: 1,
        field2: 1,
    };
    #[cfg(feature = "len_is_51_______")]
    MyStruct {
        field1: 1,
        field2: 1,
    };

    // Enum variant expression
    #[cfg(feature = "len_is_50")] MyEnum::Variant;
    #[cfg(feature = "len_is_51_")]
    MyEnum::Variant;

    // Call expression
    #[cfg(feature = "len_is_50")] some_function();
    #[cfg(feature = "len_is_51_")]
    some_function();

    // Method call expression
    #[cfg(feature = "len_is_50")] object.method();
    #[cfg(feature = "len_is_51_")]
    object.method();

    // Field access expression
    #[cfg(feature = "len_is_50")] my_struct.field;
    #[cfg(feature = "len_is_51_")]
    my_struct.field;

    // Tuple indexing expression
    #[cfg(feature = "len_is_50_____")] my_tuple.0;
    #[cfg(feature = "len_is_51______")]
    my_tuple.0;

    // Indexing expression
    #[cfg(feature = "len_is_50____")] my_array[0];
    #[cfg(feature = "len_is_51_____")]
    my_array[0];

    // Range expression
    #[cfg(feature = "len_is_50___________")]
    1..5;
    #[cfg(feature = "len_is_51____________")]
    1..5;

    // If expression
    #[cfg(feature = "len_is_50__")]
    if condition {
        1
    };
    #[cfg(feature = "len_is_51___")]
    if condition {
        1
    };

    // Loop expression
    #[cfg(feature = "len_is_50__________")]
    loop {
        break;
    }
    #[cfg(feature = "len_is_51___________")]
    loop {
        break;
    }

    // While expression
    #[cfg(feature = "len_is_50____")]
    while cond {
        break;
    }
    #[cfg(feature = "len_is_51_____")]
    while cond {
        break;
    }

    // For expression
    #[cfg(feature = "len_is_50")]
    for i in 0..10 {
        break;
    }
    #[cfg(feature = "len_is_51_")]
    for i in 0..10 {
        break;
    }

    // Match expression
    #[cfg(feature = "len_is_50___")]
    match value {
        Pattern1 => 1,
        _ => 2,
    };
    #[cfg(feature = "len_is_51____")]
    match value {
        Pattern1 => 1,
        _ => 2,
    };

    // Return expression
    #[cfg(feature = "len_is_50_________")] return;
    #[cfg(feature = "len_is_51__________")]
    return;

    // Break expression
    #[cfg(feature = "len_is_50__________")] break;
    #[cfg(feature = "len_is_51___________")]
    break;

    // Continue expression
    #[cfg(feature = "len_is_50_______")] continue;
    #[cfg(feature = "len_is_51________")]
    continue;

    // Closure expression
    #[cfg(feature = "len_is_50______")] |x| x + 1;
    #[cfg(feature = "len_is_51_______")]
    |x| x + 1;

    // Async block expression
    #[cfg(feature = "len_is_50_________")]
    async {
        #[cfg(feature = "len_50__")] future.await;
        #[cfg(feature = "len_51___")]
        future.await;
    };

    #[cfg(feature = "len_is_51__________")]
    async {
        #[cfg(feature = "len_50__")] future.await;
        #[cfg(feature = "len_51___")]
        future.await;
    };

    // Try expression
    #[cfg(feature = "len_is_50___")] some_result?;
    #[cfg(feature = "len_is_51____")]
    some_result?;
}
