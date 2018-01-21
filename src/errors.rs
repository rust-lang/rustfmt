// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::codemap::FileName;

use std::convert::From;
use std::io;
use std::path::PathBuf;

pub type RustfmtResult<T> = Result<T, RustfmtError>;

#[derive(Debug, Fail)]
pub enum RustfmtError {
    #[fail(display = "{}", _0)] IOError(io::Error),
    #[fail(display = "{}: {}", _0, _1)] FileIOError(FileName, io::Error),
    #[fail(display = "unknown config option found: `{}`", _0)] UnknownConfig(String),
    #[fail(display = "failed to find a config file for the given path `{:?}`", _0)]
    ConfigFileNotFound(PathBuf),
    #[fail(display = "failed to parse a config file: {}", _0)] ConfigFileParseError(String),
    // Since parse errors are already emitted by the parser, we do no to emit anything.
    #[fail(display = "")] ParseError,
    #[fail(display = "unstable features are only available on nightly channel")] UnstableFeature,
    #[fail(display = "invalid command line argument found for {}: {}", _0, _1)]
    InvalidCommandLineOption(String, String),
}

impl From<io::Error> for RustfmtError {
    fn from(err: io::Error) -> Self {
        RustfmtError::IOError(err)
    }
}
