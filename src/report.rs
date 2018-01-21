// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Find and report errors *after* rustfmt has run.

use syntax::codemap::FileName;
use term;

use comment::{CharClasses, FullCodeCharKind};
use config::Config;
use issues::{BadIssueSeeker, Issue};
use filemap::{append_newline, write_file};
use visitor::FmtVisitor;

use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use std::io;

/// Returns true if the line with the given line number was skipped by `#[rustfmt_skip]`.
fn is_skipped_line(line_number: usize, skipped_range: &[(usize, usize)]) -> bool {
    skipped_range
        .iter()
        .any(|&(lo, hi)| lo <= line_number && line_number <= hi)
}

#[derive(Clone, Copy)]
enum ErrorKind {
    /// Line has exceeded character limit (found, maximum)
    LineOverflow(usize, usize),
    /// Line ends in whitespace
    TrailingWhitespace,
    /// TO-DO or FIX-ME item without an issue number
    BadIssue(Issue),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ErrorKind::LineOverflow(found, maximum) => write!(
                fmt,
                "line exceeded maximum width (maximum: {}, found: {})",
                maximum, found
            ),
            ErrorKind::TrailingWhitespace => write!(fmt, "left behind trailing whitespace"),
            ErrorKind::BadIssue(issue) => write!(fmt, "found {}", issue),
        }
    }
}

/// Formatting errors that are identified *after* rustfmt has run.
pub struct FormattingError {
    line: usize,
    kind: ErrorKind,
    is_comment: bool,
    is_string: bool,
    line_buffer: String,
}

impl FormattingError {
    fn msg_prefix(&self) -> &str {
        match self.kind {
            ErrorKind::LineOverflow(..) | ErrorKind::TrailingWhitespace => "error:",
            ErrorKind::BadIssue(_) => "WARNING:",
        }
    }

    fn msg_suffix(&self) -> &str {
        if self.is_comment || self.is_string {
            "set `error_on_unformatted = false` to suppress \
             the warning against comments or string literals\n"
        } else {
            ""
        }
    }

    // (space, target)
    pub fn format_len(&self) -> (usize, usize) {
        match self.kind {
            ErrorKind::LineOverflow(found, max) => (max, found - max),
            ErrorKind::TrailingWhitespace => {
                let trailing_ws_len = self.line_buffer
                    .chars()
                    .rev()
                    .take_while(|c| c.is_whitespace())
                    .count();
                (self.line_buffer.len() - trailing_ws_len, trailing_ws_len)
            }
            _ => unreachable!(),
        }
    }
}

fn target_str(space_len: usize, target_len: usize) -> String {
    let empty_line: String = repeat(" ").take(space_len).collect();
    let overflowed: String = repeat("^").take(target_len).collect();
    empty_line + &overflowed
}

/// Maps stringified file paths to their associated formatting errors.
pub struct FormatReport {
    file_error_map: HashMap<FileName, Vec<FormattingError>>,
}

impl FormatReport {
    pub fn new() -> FormatReport {
        FormatReport {
            file_error_map: HashMap::new(),
        }
    }

    pub fn warning_count(&self) -> usize {
        self.file_error_map
            .iter()
            .map(|(_, errors)| errors.len())
            .fold(0, |acc, x| acc + x)
    }

    pub fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }

    pub fn print_warnings_fancy(
        &self,
        mut t: Box<term::Terminal<Output = io::Stderr>>,
    ) -> Result<(), term::Error> {
        for (file, errors) in &self.file_error_map {
            for error in errors {
                let prefix_space_len = error.line.to_string().len();
                let prefix_spaces: String = repeat(" ").take(1 + prefix_space_len).collect();

                // First line: the overview of error
                t.fg(term::color::RED)?;
                t.attr(term::Attr::Bold)?;
                write!(t, "{} ", error.msg_prefix())?;
                t.reset()?;
                t.attr(term::Attr::Bold)?;
                write!(t, "{}\n", error.kind)?;

                // Second line: file info
                write!(t, "{}--> ", &prefix_spaces[1..])?;
                t.reset()?;
                write!(t, "{}:{}\n", file, error.line)?;

                // Third to fifth lines: show the line which triggered error, if available.
                if !error.line_buffer.is_empty() {
                    let (space_len, target_len) = error.format_len();
                    t.attr(term::Attr::Bold)?;
                    write!(t, "{}|\n{} | ", prefix_spaces, error.line)?;
                    t.reset()?;
                    write!(t, "{}\n", error.line_buffer)?;
                    t.attr(term::Attr::Bold)?;
                    write!(t, "{}| ", prefix_spaces)?;
                    t.fg(term::color::RED)?;
                    write!(t, "{}\n", target_str(space_len, target_len))?;
                    t.reset()?;
                }

                // The last line: show note if available.
                let msg_suffix = error.msg_suffix();
                if !msg_suffix.is_empty() {
                    t.attr(term::Attr::Bold)?;
                    write!(t, "{}= note: ", prefix_spaces)?;
                    t.reset()?;
                    write!(t, "{}\n", error.msg_suffix())?;
                } else {
                    write!(t, "\n")?;
                }
                t.reset()?;
            }
        }

        if !self.file_error_map.is_empty() {
            t.attr(term::Attr::Bold)?;
            write!(t, "warning: ")?;
            t.reset()?;
            write!(
                t,
                "rustfmt may have failed to format. See previous {} errors.\n\n",
                self.warning_count(),
            )?;
        }

        Ok(())
    }
}

impl fmt::Display for FormatReport {
    // Prints all the formatting errors.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (file, errors) in &self.file_error_map {
            for error in errors {
                let prefix_space_len = error.line.to_string().len();
                let prefix_spaces: String = repeat(" ").take(1 + prefix_space_len).collect();

                let error_line_buffer = if error.line_buffer.is_empty() {
                    String::from(" ")
                } else {
                    let (space_len, target_len) = error.format_len();
                    format!(
                        "{}|\n{} | {}\n{}| {}",
                        prefix_spaces,
                        error.line,
                        error.line_buffer,
                        prefix_spaces,
                        target_str(space_len, target_len)
                    )
                };

                let error_info = format!("{} {}", error.msg_prefix(), error.kind);
                let file_info = format!("{}--> {}:{}", &prefix_spaces[1..], file, error.line);
                let msg_suffix = error.msg_suffix();
                let note = if msg_suffix.is_empty() {
                    String::new()
                } else {
                    format!("{}note= ", prefix_spaces)
                };

                write!(
                    fmt,
                    "{}\n{}\n{}\n{}{}\n",
                    error_info,
                    file_info,
                    error_line_buffer,
                    note,
                    error.msg_suffix()
                )?;
            }
        }
        if !self.file_error_map.is_empty() {
            write!(
                fmt,
                "warning: rustfmt may have failed to format. See previous {} errors.\n",
                self.warning_count(),
            )?;
        }
        Ok(())
    }
}

fn should_report_error(
    config: &Config,
    char_kind: FullCodeCharKind,
    is_string: bool,
    error_kind: ErrorKind,
) -> bool {
    let allow_error_report = if char_kind.is_comment() || is_string {
        config.error_on_unformatted()
    } else {
        true
    };

    match error_kind {
        ErrorKind::LineOverflow(..) => config.error_on_line_overflow() && allow_error_report,
        ErrorKind::TrailingWhitespace => allow_error_report,
        _ => true,
    }
}

/// Iterate over formatted text. If any error is found, add it to the report.
/// FIXME(#209) warn on bad license
/// FIXME(#20) other stuff for parity with make tidy
pub fn report_errors_in_formatted_text(
    text: &mut String,
    name: &FileName,
    skipped_range: &[(usize, usize)],
    config: &Config,
    report: &mut FormatReport,
) {
    // Iterate over the chars in the file map.
    let mut trims = vec![];
    let mut last_wspace: Option<usize> = None;
    let mut line_len = 0;
    let mut cur_line = 1;
    let mut newline_count = 0;
    let mut errors = vec![];
    let mut issue_seeker = BadIssueSeeker::new(config.report_todo(), config.report_fixme());
    let mut line_buffer = String::with_capacity(config.max_width() * 2);
    let mut is_string = false; // true if the current line contains a string literal.
    let mut format_line = config.file_lines().contains_line(name, cur_line);

    for (kind, (b, c)) in CharClasses::new(text.chars().enumerate()) {
        if c == '\r' {
            continue;
        }

        if format_line {
            // Add warnings for bad todos/ fixmes
            if let Some(issue) = issue_seeker.inspect(c) {
                errors.push(FormattingError {
                    line: cur_line,
                    kind: ErrorKind::BadIssue(issue),
                    is_comment: false,
                    is_string: false,
                    line_buffer: String::new(),
                });
            }
        }

        if c == '\n' {
            if format_line {
                // Check for (and record) trailing whitespace.
                if let Some(..) = last_wspace {
                    if should_report_error(config, kind, is_string, ErrorKind::TrailingWhitespace) {
                        trims.push((cur_line, kind, line_buffer.clone()));
                    }
                    line_len -= 1;
                }

                // Check for any line width errors we couldn't correct.
                let error_kind = ErrorKind::LineOverflow(line_len, config.max_width());
                if line_len > config.max_width() && !is_skipped_line(cur_line, skipped_range)
                    && should_report_error(config, kind, is_string, error_kind)
                {
                    errors.push(FormattingError {
                        line: cur_line,
                        kind: error_kind,
                        is_comment: kind.is_comment(),
                        is_string: is_string,
                        line_buffer: line_buffer.clone(),
                    });
                }
            }

            line_len = 0;
            cur_line += 1;
            format_line = config.file_lines().contains_line(name, cur_line);
            newline_count += 1;
            last_wspace = None;
            line_buffer.clear();
            is_string = false;
        } else {
            newline_count = 0;
            line_len += 1;
            if c.is_whitespace() {
                if last_wspace.is_none() {
                    last_wspace = Some(b);
                }
            } else {
                last_wspace = None;
            }
            line_buffer.push(c);
            if kind.is_string() {
                is_string = true;
            }
        }
    }

    if newline_count > 1 {
        debug!("track truncate: {} {}", text.len(), newline_count);
        let line = text.len() - newline_count + 1;
        text.truncate(line);
    }

    for &(l, kind, ref b) in &trims {
        if !is_skipped_line(l, skipped_range) {
            errors.push(FormattingError {
                line: l,
                kind: ErrorKind::TrailingWhitespace,
                is_comment: kind.is_comment(),
                is_string: kind.is_string(),
                line_buffer: b.clone(),
            });
        }
    }

    report.file_error_map.insert(name.clone(), errors);
}

impl<'a> FmtVisitor<'a> {
    pub fn report_errors_after_format<T: io::Write>(
        &mut self,
        file_name: &FileName,
        report: &mut FormatReport,
        out: &mut Option<T>,
    ) -> Result<bool, io::Error> {
        // For some reason, the codemap does not include terminating
        // newlines so we must add one on for each file. This is sad.
        append_newline(&mut self.buffer);
        report_errors_in_formatted_text(
            &mut self.buffer,
            file_name,
            &self.skipped_range,
            self.config,
            report,
        );
        out.as_mut().map_or(Ok(false), |out| {
            write_file(&self.buffer, file_name, out, self.config)
        })
    }
}
