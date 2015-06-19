// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Build script. Just copies default.toml from the src to the target dir.

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::convert::AsRef;
use std::io::Read;

fn main() {
    let in_file = Path::new("src/default.toml");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut out_file = PathBuf::new();
    out_file.push(manifest_dir);
    out_file.push("default.toml");

    if !files_identical(&out_file, in_file) {
        std::fs::copy(in_file, out_file).unwrap();
    }
}

fn files_identical<P: AsRef<Path>, Q: AsRef<Path>>(lhs: P, rhs: Q) -> bool {
    match (File::open(lhs), File::open(rhs)) {
        (Ok(mut f1), Ok(mut f2)) => {
            let mut buf1 = Vec::new();
            let mut buf2 = Vec::new();
            f1.read_to_end(&mut buf1).unwrap();
            f2.read_to_end(&mut buf2).unwrap();
            buf1 == buf2
        },
        _ => false
    }
}
