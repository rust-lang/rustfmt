// rustfmt-inline_attribute_width: 50

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
