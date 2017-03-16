// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use rustfmt_diff::{Mismatch, DiffLine};
use std::io::{self, Write};

pub fn output_codeclimate<T>(mut writer: T,
                             filename: &str,
                             diff: Vec<Mismatch>)
                             -> Result<(), io::Error>
    where T: Write
{
    for mismatch in diff {
        for line in mismatch.lines {
            if let DiffLine::Expected(ref exp) = line {
                try!(write!(writer,
                            "{{\"type\": \"issue\",\
                    \"check_name\":  \"Style\",\
                    \"description\": \"Should be `{exp}`\",\
                    \"categories\":  [\"Style\"],\
                    \"location\": {{\
                        \"path\": \"{filename}\",\
                        \"lines\": {{\
                            \"begin\": {line},\
                            \"end\":   {line}\
                        }}\
                    }}\
                }}\r\n\0",
                            filename = filename,
                            exp = json_escape_str(exp),
                            line = mismatch.line_number));
            }
        }
    }
    Ok(())
}

fn json_escape_str(string: &str) -> String {
    let mut out = String::new();
    for c in string.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out
}
