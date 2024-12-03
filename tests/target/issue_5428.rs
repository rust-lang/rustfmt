// cfg-if: version 1.0.0
cfg_if::cfg_if! {
    if #[cfg(windows)] {
        compile_error!{"..."};
    }
}
