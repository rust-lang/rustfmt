// rustfmt-version: One

#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(nll)]
#![feature(panic_unwind)]
#![feature(staged_api)]
#![feature(std_internals)]
#![feature(unwind_attributes)]
#![feature(abi_thiscall)]
#![feature(rustc_attrs)]
#![feature(raw)]
#![panic_runtime]
#![feature(panic_runtime)]

// `real_imp` is unused with Miri, so silence warnings.
#![cfg_attr(miri, allow(dead_code))]
