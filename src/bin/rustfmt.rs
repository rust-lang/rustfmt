// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(not(test))]
#![feature(path_ext)]

extern crate rustfmt;

use rustfmt::{WriteMode, run};

use std::env;
use std::fs::{File, PathExt};
use std::io::{self, Read, Error, ErrorKind};
use std::path::PathBuf;

fn lookup_config_file() -> io::Result<PathBuf> {
    let mut current = try!(env::current_dir());
    loop {
        let config_file = current.join("default.toml");
        if config_file.exists() {
            return Ok(config_file);
        } else {
            current = match current.parent() {
                // if the current directory has no parent, we're done searching
                None => return Err(Error::new(ErrorKind::NotFound, "config not found")),
                Some(path) => path.to_path_buf(),
            };
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let config_path = lookup_config_file().unwrap();
    let mut def_config_file = File::open(config_path).unwrap();
    let mut def_config = String::new();
    def_config_file.read_to_string(&mut def_config).unwrap();

    run(args, WriteMode::Overwrite, &def_config);

    std::process::exit(0);
}
