// rustfmt-config: issue-3933.toml
#[cfg_attr(windows, path = "imp-a.rs")]
#[cfg_attr(not(windows), path = "imp-b.rs")]
mod imp;
pub use imp::A;
