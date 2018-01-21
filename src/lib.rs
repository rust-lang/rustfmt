// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(match_default_bindings)]
#![feature(rustc_private)]
#![feature(type_ascription)]

#[macro_use]
extern crate derive_new;
extern crate diff;
#[macro_use]
extern crate log;
extern crate regex;
extern crate rustc_errors as errors;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate syntax;
extern crate term;
extern crate unicode_segmentation;

use std::io::{self, stdout, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;

use errors::{DiagnosticBuilder, Handler};
use errors::emitter::{ColorConfig, EmitterWriter};
use syntax::ast;
use syntax::codemap::{CodeMap, FilePathMapping};
pub use syntax::codemap::FileName;
use syntax::parse::{self, ParseSess};

use checkstyle::{output_footer, output_header};
pub use config::Config;
use filemap::FileMap;
use shape::Indent;
use utils::use_colored_tty;
use visitor::{FmtVisitor, SnippetProvider};
pub use report::*;

pub use self::summary::Summary;

#[macro_use]
mod utils;
mod shape;
mod spanned;
pub mod config;
pub mod codemap;
pub mod filemap;
pub mod file_lines;
pub mod visitor;
mod checkstyle;
mod closures;
mod items;
mod missed_spans;
mod lists;
mod types;
mod expr;
mod imports;
mod issues;
mod rewrite;
mod string;
mod comment;
pub mod modules;
pub mod rustfmt_diff;
mod chains;
mod macros;
mod patterns;
mod summary;
mod vertical;
mod report;

// Formatting which depends on the AST.
fn format_ast<T: Write>(
    krate: &ast::Crate,
    parse_session: &mut ParseSess,
    main_file: &FileName,
    config: &Config,
    mut out: &mut Option<&mut T>,
) -> Result<(FileMap, bool, FormatReport), io::Error> {
    let mut result = FileMap::new();
    let mut report = FormatReport::new();
    // diff mode: check if any files are differing
    let mut has_diff = false;

    // We always skip children for the "Plain" write mode, since there is
    // nothing to distinguish the nested module contents.
    let skip_children = config.skip_children() || config.write_mode() == config::WriteMode::Plain;
    for (file_name, module) in modules::list_files(krate, parse_session.codemap())? {
        if skip_children && file_name != *main_file {
            continue;
        }
        if config.verbose() {
            println!("Formatting {}", file_name);
        }
        let filemap = parse_session
            .codemap()
            .lookup_char_pos(module.inner.lo())
            .file;
        let big_snippet = filemap.src.as_ref().unwrap();
        let snippet_provider = SnippetProvider::new(filemap.start_pos, big_snippet);
        let mut visitor = FmtVisitor::from_codemap(parse_session, config, &snippet_provider);
        // Format inner attributes if available.
        if !krate.attrs.is_empty() && file_name == *main_file {
            visitor.skip_empty_lines(filemap.end_pos);
            if visitor.visit_attrs(&krate.attrs, ast::AttrStyle::Inner) {
                visitor.push_rewrite(module.inner, None);
            } else {
                visitor.format_separate_mod(module, &*filemap);
            }
        } else {
            visitor.last_pos = filemap.start_pos;
            visitor.skip_empty_lines(filemap.end_pos);
            visitor.format_separate_mod(module, &*filemap);
        };

        assert_eq!(
            visitor.line_number,
            ::utils::count_newlines(&format!("{}", visitor.buffer))
        );

        let maybe_has_diff = visitor.report_errors_after_format(&file_name, &mut report, &mut out);

        has_diff |= match maybe_has_diff {
            Ok(diff) => diff,
            Err(e) => {
                // Create a new error with file name to help users see which files failed
                let err_msg = format!("{}: {}", file_name, e);
                return Err(io::Error::new(e.kind(), err_msg));
            }
        };

        result.push((file_name, visitor.buffer));
    }

    Ok((result, has_diff, report))
}

#[derive(Debug)]
pub enum Input {
    File(PathBuf),
    Text(String),
}

fn parse_input(
    input: Input,
    parse_session: &ParseSess,
) -> Result<ast::Crate, Option<DiagnosticBuilder>> {
    let result = match input {
        Input::File(file) => {
            let mut parser = parse::new_parser_from_file(parse_session, &file);
            parser.cfg_mods = false;
            parser.parse_crate_mod()
        }
        Input::Text(text) => {
            let mut parser = parse::new_parser_from_source_str(
                parse_session,
                FileName::Custom("stdin".to_owned()),
                text,
            );
            parser.cfg_mods = false;
            parser.parse_crate_mod()
        }
    };

    match result {
        Ok(c) => {
            if parse_session.span_diagnostic.has_errors() {
                // Bail out if the parser recovered from an error.
                Err(None)
            } else {
                Ok(c)
            }
        }
        Err(e) => Err(Some(e)),
    }
}

pub fn format_input<T: Write>(
    input: Input,
    config: &Config,
    mut out: Option<&mut T>,
) -> Result<(Summary, FileMap, FormatReport), (io::Error, Summary)> {
    let mut summary = Summary::default();
    if config.disable_all_formatting() {
        // When the input is from stdin, echo back the input.
        if let Input::Text(ref buf) = input {
            if let Err(e) = io::stdout().write_all(buf.as_bytes()) {
                return Err((e, summary));
            }
        }
        return Ok((summary, FileMap::new(), FormatReport::new()));
    }
    let codemap = Rc::new(CodeMap::new(FilePathMapping::empty()));

    let tty_handler = if config.hide_parse_errors() {
        let silent_emitter = Box::new(EmitterWriter::new(
            Box::new(Vec::new()),
            Some(codemap.clone()),
            false,
        ));
        Handler::with_emitter(true, false, silent_emitter)
    } else {
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(codemap.clone()))
    };
    let mut parse_session = ParseSess::with_span_handler(tty_handler, codemap.clone());

    let main_file = match input {
        Input::File(ref file) => FileName::Real(file.clone()),
        Input::Text(..) => FileName::Custom("stdin".to_owned()),
    };

    let krate = match parse_input(input, &parse_session) {
        Ok(krate) => {
            if parse_session.span_diagnostic.has_errors() {
                summary.add_parsing_error();
            }
            krate
        }
        Err(diagnostic) => {
            if let Some(mut diagnostic) = diagnostic {
                diagnostic.emit();
            }
            summary.add_parsing_error();
            return Ok((summary, FileMap::new(), FormatReport::new()));
        }
    };

    summary.mark_parse_time();

    // Suppress error output after parsing.
    let silent_emitter = Box::new(EmitterWriter::new(
        Box::new(Vec::new()),
        Some(codemap.clone()),
        false,
    ));
    parse_session.span_diagnostic = Handler::with_emitter(true, false, silent_emitter);

    let format_result = format_ast(&krate, &mut parse_session, &main_file, config, &mut out);

    summary.mark_format_time();

    if config.verbose() {
        fn duration_to_f32(d: Duration) -> f32 {
            d.as_secs() as f32 + d.subsec_nanos() as f32 / 1_000_000_000f32
        }

        println!(
            "Spent {0:.3} secs in the parsing phase, and {1:.3} secs in the formatting phase",
            duration_to_f32(summary.get_parse_time().unwrap()),
            duration_to_f32(summary.get_format_time().unwrap()),
        );
    }

    match format_result {
        Ok((file_map, has_diff, report)) => {
            if report.has_warnings() {
                summary.add_formatting_error();
            }

            if has_diff {
                summary.add_diff();
            }

            Ok((summary, file_map, report))
        }
        Err(e) => Err((e, summary)),
    }
}

/// An entry point to rustfmt.
pub fn run(input: Input, config: &Config) -> Summary {
    let out = &mut stdout();
    output_header(out, config.write_mode()).ok();
    match format_input(input, config, Some(out)) {
        Ok((summary, _, report)) => {
            output_footer(out, config.write_mode()).ok();

            if report.has_warnings() {
                match term::stderr() {
                    Some(ref t)
                        if use_colored_tty(config.color()) && t.supports_color()
                            && t.supports_attr(term::Attr::Bold) =>
                    {
                        match report.print_warnings_fancy(term::stderr().unwrap()) {
                            Ok(..) => (),
                            Err(..) => panic!("Unable to write to stderr: {}", report),
                        }
                    }
                    _ => msg!("{}", report),
                }
            }

            summary
        }
        Err((msg, mut summary)) => {
            msg!("Error writing files: {}", msg);
            summary.add_operational_error();
            summary
        }
    }
}

/// Format the given snippet. The snippet is expected to be *complete* code.
/// When we cannot parse the given snippet, this function returns `None`.
pub fn format_snippet(snippet: &str, config: &Config) -> Option<String> {
    let mut out: Vec<u8> = Vec::with_capacity(snippet.len() * 2);
    let input = Input::Text(snippet.into());
    let mut config = config.clone();
    config.set().write_mode(config::WriteMode::Plain);
    config.set().hide_parse_errors(true);
    match format_input(input, &config, Some(&mut out)) {
        // `format_input()` returns an empty string on parsing error.
        Ok(..) if out.is_empty() && !snippet.is_empty() => None,
        Ok(..) => String::from_utf8(out).ok(),
        Err(..) => None,
    }
}

/// Format the given code block. Mainly targeted for code block in comment.
/// The code block may be incomplete (i.e. parser may be unable to parse it).
/// To avoid panic in parser, we wrap the code block with a dummy function.
/// The returned code block does *not* end with newline.
pub fn format_code_block(code_snippet: &str, config: &Config) -> Option<String> {
    // Wrap the given code block with `fn main()` if it does not have one.
    let fn_main_prefix = "fn main() {\n";
    let snippet = fn_main_prefix.to_owned() + code_snippet + "\n}";

    // Trim "fn main() {" on the first line and "}" on the last line,
    // then unindent the whole code block.
    format_snippet(&snippet, config).map(|s| {
        // 2 = "}\n"
        s[fn_main_prefix.len()..s.len().checked_sub(2).unwrap_or(0)]
            .lines()
            .map(|line| {
                if line.len() > config.tab_spaces() {
                    // Make sure that the line has leading whitespaces.
                    let indent_str =
                        Indent::from_width(config, config.tab_spaces()).to_string(config);
                    if line.starts_with(indent_str.as_ref()) {
                        &line[config.tab_spaces()..]
                    } else {
                        line
                    }
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    })
}

#[cfg(test)]
mod test {
    use super::{format_code_block, format_snippet, Config};

    #[test]
    fn test_no_panic_on_format_snippet_and_format_code_block() {
        // `format_snippet()` and `format_code_block()` should not panic
        // even when we cannot parse the given snippet.
        let snippet = "let";
        assert!(format_snippet(snippet, &Config::default()).is_none());
        assert!(format_code_block(snippet, &Config::default()).is_none());
    }

    fn test_format_inner<F>(formatter: F, input: &str, expected: &str) -> bool
    where
        F: Fn(&str, &Config) -> Option<String>,
    {
        let output = formatter(input, &Config::default());
        output.is_some() && output.unwrap() == expected
    }

    #[test]
    fn test_format_snippet() {
        let snippet = "fn main() { println!(\"hello, world\"); }";
        let expected = "fn main() {\n    \
                        println!(\"hello, world\");\n\
                        }\n";
        assert!(test_format_inner(format_snippet, snippet, expected));
    }

    #[test]
    fn test_format_code_block() {
        // simple code block
        let code_block = "let x=3;";
        let expected = "let x = 3;";
        assert!(test_format_inner(format_code_block, code_block, expected));

        // more complex code block, taken from chains.rs.
        let code_block =
"let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
context.config.indent_style() == IndentStyle::Visual || is_small_parent,
)
} else if is_block_expr(context, &parent, &parent_rewrite) {
match context.config.indent_style() {
// Try to put the first child on the same line with parent's last line
IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
// The parent is a block, so align the rest of the chain with the closing
// brace.
IndentStyle::Visual => (parent_shape, false),
}
} else {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
false,
)
};
";
        let expected =
"let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        context.config.indent_style() == IndentStyle::Visual || is_small_parent,
    )
} else if is_block_expr(context, &parent, &parent_rewrite) {
    match context.config.indent_style() {
        // Try to put the first child on the same line with parent's last line
        IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
        // The parent is a block, so align the rest of the chain with the closing
        // brace.
        IndentStyle::Visual => (parent_shape, false),
    }
} else {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        false,
    )
};";
        assert!(test_format_inner(format_code_block, code_block, expected));
    }
}
