// rustfmt-version: One

// Making sure trailing commas are not removed from attributes (no need for test source)
#![cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always,))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always,),)]
#![cfg_attr(feature = "cargo-clippy",)]
#![allow(nline_always,)]
