// Tests that where a single file is referred to in multiple places, we don't
// crash.

#[cfg(all(foo))]
#[path = "closure_block_style.rs"]
pub mod imp;

#[cfg(all(bar))]
#[path = "closure_block_style.rs"]
pub mod imp;
