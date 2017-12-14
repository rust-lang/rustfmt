// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Add `$(rustc --print sysroot)` to the rpath.
    let lib_path = get_sysroot_lib_path();
    let rust_flags = format!("-Clink-args=-Xlinker -rpath={}", lib_path);
    println!("cargo:rustc-env=RUSTFLAGS=\"{}\"", rust_flags);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out_dir.join("commit-info.txt"))
        .unwrap()
        .write_all(commit_info().as_bytes())
        .unwrap();
}

fn get_sysroot_lib_path() -> String {
    let stdout = Command::new("rustc")
        .args(&["--print", "sysroot"])
        .output()
        .expect("rustc --print sysroot failed")
        .stdout;
    let sysroot_path = String::from_utf8(stdout).unwrap();
    format!("{}/lib", sysroot_path.trim_right()) // Trim a trailing newline.
}

// Try to get hash and date of the last commit on a best effort basis. If anything goes wrong
// (git not installed or if this is not a git repository) just return an empty string.
fn commit_info() -> String {
    match (commit_hash(), commit_date()) {
        (Some(hash), Some(date)) => format!(" ({} {})", hash.trim_right(), date),
        _ => String::new(),
    }
}

fn commit_hash() -> Option<String> {
    Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

fn commit_date() -> Option<String> {
    Command::new("git")
        .args(&["log", "-1", "--date=short", "--pretty=format:%cd"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}
