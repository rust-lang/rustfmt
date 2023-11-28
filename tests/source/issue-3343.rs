// rustfmt-inline_attribute_width: 50

#[cfg(feature = "alloc")]
use core::slice;

#[cfg(feature = "alloc")]
use total_len_is::_50__;

#[cfg(feature = "alloc")]
use total_len_is::_51___;

#[cfg(feature = "alloc")]
extern crate len_is_50_;

#[cfg(feature = "alloc")]
extern crate len_is_51__;

// https://github.com/rust-lang/rustfmt/issues/3343#issuecomment-589945611
extern "C" {
    #[no_mangle]
    fn foo();
}

fn main() {
    #[cfg(feature = "alloc")]
    foo();
    #[cfg(feature = "alloc")]
    {
        foo();
    }
    {
        #[cfg(feature = "alloc")]
        foo();
    }
}

// https://github.com/rust-lang/rustfmt/pull/5538#issuecomment-1272367684
struct EventConfigWidget {
    #[widget]
    menu_delay: Spinner<u32>,
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
