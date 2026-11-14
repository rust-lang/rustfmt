#[cfg_attr(unix, path = "meow_unix")]
#[cfg_attr(windows, path = "meow_windows")]
#[cfg_attr(wasm, path = "meow/wasm")]
mod meow {
    mod mrrp;

    #[path = "y.rs"]
    mod x;

    #[path = "dog"]
    mod bark {
        mod woof;
    }
}
