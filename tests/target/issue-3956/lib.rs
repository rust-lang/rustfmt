// rustfmt-config: issue-3956.toml
// rustfmt-recursive: true
#[cfg_attr(windows, path = "graphics.rs")]
#[cfg_attr(not(windows), path = "graphics_emu.rs")]
mod graphics;
