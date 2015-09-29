// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Format string literals.

use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;

use Indent;
use config::Config;
use utils::round_up_to_power_of_two;

use MIN_STRING;

pub struct StringFormat<'a> {
    pub opener: &'a str,
    pub closer: &'a str,
    pub line_start: &'a str,
    pub line_end: &'a str,
    pub width: usize,
    pub offset: Indent,
    pub trim_end: bool,
    pub config: &'a Config,
}

// TODO: simplify this!
pub fn rewrite_string<'a>(s: &str, fmt: &StringFormat<'a>) -> Option<String> {
    // TODO if lo.col > IDEAL - 10, start a new line (need cur indent for that)
    // Strip line breaks.
    let re = Regex::new(r"(\\[:space:]+)").unwrap();
    let stripped_str = re.replace_all(s, "");

    let graphemes = UnicodeSegmentation::graphemes(&*stripped_str, false).collect::<Vec<&str>>();
    let indent = fmt.offset.to_string(fmt.config);

    let mut cur_start = 0;
    let mut result = String::with_capacity(round_up_to_power_of_two(s.len()));
    result.push_str(fmt.opener);

    let ender_length = fmt.line_end.len();
    // If we cannot put at least a single character per line, the rewrite won't
    // succeed.
    let max_chars = try_opt!(fmt.width.checked_sub(fmt.opener.len() + ender_length + 1)) + 1;

    loop {
        let mut cur_end = cur_start + max_chars;

        if cur_end >= graphemes.len() {
            let line = &graphemes[cur_start..].join("");
            result.push_str(line);
            break;
        }

        // Push cur_end left until we reach whitespace.
        while !graphemes[cur_end - 1].trim().is_empty() {
            cur_end -= 1;
            if cur_end - cur_start < MIN_STRING {
                // We can't break at whitespace, fall back to splitting
                // anywhere that doesn't break an escape sequence.
                cur_end = backwards_find_whole_non_escaped(&graphemes, cur_start + max_chars);
                break;
            }
        }
        // Make sure there is no whitespace to the right of the break.
        while cur_end < s.len() && graphemes[cur_end].trim().is_empty() {
            cur_end += 1;
        }
        let raw_line = graphemes[cur_start..cur_end].join("");
        let line = if fmt.trim_end {
            raw_line.trim()
        } else {
            // TODO: use as_str once it's stable.
            &*raw_line
        };

        result.push_str(line);
        result.push_str(fmt.line_end);
        result.push('\n');
        result.push_str(&indent);
        result.push_str(fmt.line_start);

        cur_start = cur_end;
    }
    result.push_str(fmt.closer);

    Some(result)
}

fn backwards_find_whole_non_escaped(graphemes: &Vec<&str>, end: usize) -> usize {
    // Trivial: \n \r \t \" \'
    // Hard: \x## \u{######}
    // Hardest:\\\\\\\\ <- break at the even gap of one of these
    use std::cmp;
    // At worst, an escape sequence can begin 9 characters before end:
    //              \u{ABCDEF}
    // window_start ^        ^ end
    let window_start: usize = cmp::max((end as isize)-9, 0) as usize;
    graphemes[window_start..end]
        .iter()
        .rposition(|s| *s == "\\")
        .map(|eidx| window_start + eidx)
        .map(|escape_idx| { // Shift the start idx if it has an odd number of \\ escapes
            if graphemes[escape_idx - 1] == "\\" {
                match graphemes[..escape_idx]
                          .iter()
                          .rposition(|s| *s != "\\")
                          .map(|i| i + 1)
                          .unwrap_or(0) {
                    i if (escape_idx - i) % 2 == 1 => escape_idx - 1,
                    _ => escape_idx,
                }
            } else {
                escape_idx
            }
        })
        .map(|escape_idx| { // convert the true idx to the end of the escape sequence
            if escape_idx == end - 1 {
                (escape_idx, 2) // Escape char is beyond end, break at end always
            } else {
                match graphemes[escape_idx + 1].chars().next().unwrap() {
                    'x' => (escape_idx, 4),
                    'u' => (escape_idx,
                            graphemes[escape_idx..]
                                .iter()
                                .position(|s| *s == "}")
                                .unwrap_or(end) + 1),
                    _ => (escape_idx, 2),
                }
            }
        })
        .map(|(escape_idx, escape_len)| {
            if escape_idx + escape_len > end {
                escape_idx // Break before this escape sequence
            } else {
                end // This sequence doesnt reach end, break at end
            }
        })
        .unwrap_or(end) // No escape char was found in the window, safe to break at end
}

#[cfg(test)]
mod test {
    use super::{StringFormat, rewrite_string};

    #[test]
    fn issue343() {
        let config = Default::default();
        let fmt = StringFormat {
            opener: "\"",
            closer: "\"",
            line_start: " ",
            line_end: "\\",
            width: 2,
            offset: ::Indent::empty(),
            trim_end: false,
            config: &config,
        };

        rewrite_string("eq_", &fmt);
    }
}
