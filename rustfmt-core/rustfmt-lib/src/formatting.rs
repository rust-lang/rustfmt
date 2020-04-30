// High level formatting functions.

use std::io;
use std::time::{Duration, Instant};

use rustc_ast::ast;

use self::newline_style::apply_newline_style;
use crate::comment::{CharClasses, FullCodeCharKind};
use crate::config::{Config, FileName};
use crate::syntux::parser::{DirectoryOwnership, Parser, ParserError};
use crate::syntux::session::ParseSess;
use crate::utils::count_newlines;
use crate::visitor::FmtVisitor;
use crate::{
    modules, source_file, ErrorKind, FormatError, FormatReport, FormatResult, Input,
    NonFormattedRange, OperationError, Session, Verbosity,
};

mod newline_style;

impl Session {
    pub(crate) fn format_input_inner(
        &mut self,
        input: Input,
        config: &Config,
    ) -> Result<FormatReport, OperationError> {
        if !config.version_meets_requirement() {
            return Err(OperationError::VersionMismatch);
        }

        rustc_ast::with_globals(config.edition().into(), || {
            self.format_project(input, config)
        })
    }

    fn format_project(
        &mut self,
        input: Input,
        config: &Config,
    ) -> Result<FormatReport, OperationError> {
        let mut timer = Timer::start();

        let format_report = FormatReport::new();

        let main_file = input.file_name();
        let input_is_stdin = main_file == FileName::Stdin;

        let mut parse_session = ParseSess::new(config)?;
        if !self.recursive && parse_session.ignore_file(&main_file) {
            format_report.add_ignored_file(main_file);
            return Ok(format_report);
        }

        // Parse the crate.
        let directory_ownership = input.to_directory_ownership();
        let original_snippet = if let Input::Text(ref str) = input {
            Some(str.to_owned())
        } else {
            None
        };

        let krate = match Parser::parse_crate(config, input, directory_ownership, &parse_session) {
            Ok(krate) => krate,
            Err(e) => {
                return Err(OperationError::ParseError {
                    input: main_file,
                    is_panic: e == ParserError::ParsePanicError,
                });
            }
        };
        timer = timer.done_parsing();

        // Suppress error output if we have to do any further parsing.
        parse_session.set_silent_emitter();

        let files = modules::ModResolver::new(
            &parse_session,
            directory_ownership.unwrap_or(DirectoryOwnership::UnownedViaMod),
            !input_is_stdin && self.recursive,
        )
        .visit_crate(&krate)
        .map_err(|e| OperationError::IoError(io::Error::new(io::ErrorKind::Other, e)))?;

        for (path, module) in files {
            let should_ignore = !input_is_stdin && parse_session.ignore_file(&path);
            if (!self.recursive && path != main_file) || should_ignore {
                continue;
            }
            should_emit_verbose(input_is_stdin, self.verbosity, || {
                println!("Formatting {}", path)
            });
            let is_root = path == main_file;
            self.format_file(
                &parse_session,
                config,
                &krate,
                path,
                &module,
                is_root,
                &format_report,
                original_snippet.clone(),
            );
        }
        timer = timer.done_formatting();

        should_emit_verbose(input_is_stdin, self.verbosity, || {
            println!(
                "Spent {0:.3} secs in the parsing phase, and {1:.3} secs in the formatting phase",
                timer.get_parse_time(),
                timer.get_format_time(),
            )
        });

        Ok(format_report)
    }
}

impl Session {
    // Formats a single file/module.
    fn format_file(
        &mut self,
        parse_session: &ParseSess,
        config: &Config,
        krate: &ast::Crate,
        path: FileName,
        module: &ast::Mod,
        is_root: bool,
        report: &FormatReport,
        original_snippet: Option<String>,
    ) {
        let snippet_provider = parse_session.snippet_provider(module.inner);
        let mut visitor =
            FmtVisitor::from_parse_sess(&parse_session, config, &snippet_provider, report.clone());
        visitor.skip_context.update_with_attrs(&krate.attrs);

        // Format inner attributes if available.
        if !krate.attrs.is_empty() && is_root {
            visitor.skip_empty_lines(snippet_provider.end_pos());
            if visitor.visit_attrs(&krate.attrs, ast::AttrStyle::Inner) {
                visitor.push_rewrite(module.inner, None);
            } else {
                visitor.format_separate_mod(module, snippet_provider.end_pos());
            }
        } else {
            visitor.last_pos = snippet_provider.start_pos();
            visitor.skip_empty_lines(snippet_provider.end_pos());
            visitor.format_separate_mod(module, snippet_provider.end_pos());
        };

        debug_assert_eq!(
            visitor.line_number,
            count_newlines(&visitor.buffer),
            "failed in format_file visitor.buffer:\n {:?}",
            &visitor.buffer
        );

        // For some reason, the source_map does not include terminating
        // newlines so we must add one on for each file. This is sad.
        source_file::append_newline(&mut visitor.buffer);

        format_lines(
            &mut visitor.buffer,
            &path,
            &visitor.skipped_range.borrow(),
            config,
            report.clone(),
        );

        apply_newline_style(
            config.newline_style(),
            &mut visitor.buffer,
            snippet_provider.entire_snippet(),
        );

        if visitor.macro_rewrite_failure {
            report.add_macro_format_failure(path.clone());
        }
        let format_result = FormatResult::success(
            visitor.buffer.to_owned(),
            visitor.skipped_range.borrow().clone(),
            original_snippet,
            config.newline_style(),
        );
        report.add_format_result(path, format_result);
    }
}

#[derive(Clone, Copy, Debug)]
enum Timer {
    Disabled,
    Initialized(Instant),
    DoneParsing(Instant, Instant),
    DoneFormatting(Instant, Instant, Instant),
}

impl Timer {
    fn start() -> Timer {
        if cfg!(target_arch = "wasm32") {
            Timer::Disabled
        } else {
            Timer::Initialized(Instant::now())
        }
    }
    fn done_parsing(self) -> Self {
        match self {
            Timer::Disabled => Timer::Disabled,
            Timer::Initialized(init_time) => Timer::DoneParsing(init_time, Instant::now()),
            _ => panic!("Timer can only transition to DoneParsing from Initialized state"),
        }
    }

    fn done_formatting(self) -> Self {
        match self {
            Timer::Disabled => Timer::Disabled,
            Timer::DoneParsing(init_time, parse_time) => {
                Timer::DoneFormatting(init_time, parse_time, Instant::now())
            }
            _ => panic!("Timer can only transition to DoneFormatting from DoneParsing state"),
        }
    }

    /// Returns the time it took to parse the source files in seconds.
    fn get_parse_time(&self) -> f32 {
        match *self {
            Timer::Disabled => panic!("this platform cannot time execution"),
            Timer::DoneParsing(init, parse_time) | Timer::DoneFormatting(init, parse_time, _) => {
                // This should never underflow since `Instant::now()` guarantees monotonicity.
                Self::duration_to_f32(parse_time.duration_since(init))
            }
            Timer::Initialized(..) => unreachable!(),
        }
    }

    /// Returns the time it took to go from the parsed AST to the formatted output. Parsing time is
    /// not included.
    fn get_format_time(&self) -> f32 {
        match *self {
            Timer::Disabled => panic!("this platform cannot time execution"),
            Timer::DoneFormatting(_init, parse_time, format_time) => {
                Self::duration_to_f32(format_time.duration_since(parse_time))
            }
            Timer::DoneParsing(..) | Timer::Initialized(..) => unreachable!(),
        }
    }

    fn duration_to_f32(d: Duration) -> f32 {
        d.as_secs() as f32 + d.subsec_nanos() as f32 / 1_000_000_000f32
    }
}

// Formatting done on a char by char or line by line basis.
// FIXME(#20): other stuff for parity with make tidy.
fn format_lines(
    text: &mut String,
    name: &FileName,
    skipped_range: &[NonFormattedRange],
    config: &Config,
    report: FormatReport,
) {
    let mut formatter = FormatLines::new(name, skipped_range, config);
    if let Some(false) = formatter.check_license(text) {
        report.add_license_failure(name.clone());
    }
    formatter.iterate(text);

    if formatter.newline_count > 1 {
        debug!("track truncate: {} {}", text.len(), formatter.newline_count);
        let line = text.len() - formatter.newline_count + 1;
        text.truncate(line);
    }

    report.append_errors(name.clone(), formatter.errors.into_iter());
}

struct FormatLines<'a> {
    name: &'a FileName,
    skipped_range: &'a [NonFormattedRange],
    last_was_space: bool,
    line_len: usize,
    cur_line: usize,
    newline_count: usize,
    errors: Vec<FormatError>,
    line_buffer: String,
    current_line_contains_string_literal: bool,
    format_line: bool,
    config: &'a Config,
}

impl<'a> FormatLines<'a> {
    fn new(
        name: &'a FileName,
        skipped_range: &'a [NonFormattedRange],
        config: &'a Config,
    ) -> FormatLines<'a> {
        FormatLines {
            name,
            skipped_range,
            last_was_space: false,
            line_len: 0,
            cur_line: 1,
            newline_count: 0,
            errors: vec![],
            line_buffer: String::with_capacity(config.max_width() * 2),
            current_line_contains_string_literal: false,
            format_line: config.file_lines().contains_line(name, 1),
            config,
        }
    }

    fn check_license(&mut self, text: &str) -> Option<bool> {
        self.config
            .license_template
            .as_ref()
            .map(|license_template| license_template.is_match(text))
    }

    // Iterate over the chars in the file map.
    fn iterate(&mut self, text: &str) {
        for (kind, c) in CharClasses::new(text.chars()) {
            if c == '\r' {
                continue;
            }

            if c == '\n' {
                self.new_line(kind);
            } else {
                self.char(c, kind);
            }
        }
    }

    fn new_line(&mut self, kind: FullCodeCharKind) {
        if self.format_line {
            // Check for (and record) trailing whitespace.
            if self.last_was_space {
                if self.should_report_error(kind, &ErrorKind::TrailingWhitespace)
                    && !self.is_skipped_line()
                {
                    self.push_err(ErrorKind::TrailingWhitespace);
                }
                self.line_len -= 1;
            }

            // Check for any line width errors we couldn't correct.
            let error_kind = ErrorKind::LineOverflow(self.line_len, self.config.max_width());
            if self.line_len > self.config.max_width()
                && !self.is_skipped_line()
                && self.should_report_error(kind, &error_kind)
            {
                self.push_err(error_kind);
            }
        }

        self.line_len = 0;
        self.cur_line += 1;
        self.format_line = self
            .config
            .file_lines()
            .contains_line(self.name, self.cur_line);
        self.newline_count += 1;
        self.last_was_space = false;
        self.line_buffer.clear();
        self.current_line_contains_string_literal = false;
    }

    fn char(&mut self, c: char, kind: FullCodeCharKind) {
        self.newline_count = 0;
        self.line_len += if c == '\t' {
            self.config.tab_spaces()
        } else {
            1
        };
        self.last_was_space = c.is_whitespace();
        self.line_buffer.push(c);
        if kind.is_string() {
            self.current_line_contains_string_literal = true;
        }
    }

    fn push_err(&mut self, kind: ErrorKind) {
        self.errors.push(FormatError::new(
            kind,
            self.cur_line,
            self.line_buffer.clone(),
        ));
    }

    fn should_report_error(&self, char_kind: FullCodeCharKind, error_kind: &ErrorKind) -> bool {
        let allow_error_report =
            if char_kind.is_comment() || self.current_line_contains_string_literal {
                self.config.error_on_unformatted()
            } else {
                true
            };

        match error_kind {
            ErrorKind::LineOverflow(..) => {
                self.config.error_on_line_overflow() && allow_error_report
            }
            ErrorKind::TrailingWhitespace => allow_error_report,
            _ => true,
        }
    }

    /// Returns `true` if the line with the given line number was skipped by `#[rustfmt::skip]`.
    fn is_skipped_line(&self) -> bool {
        self.skipped_range
            .iter()
            .any(|range| range.contains(self.cur_line))
    }
}

fn should_emit_verbose<F>(forbid_verbose_output: bool, verbosity: Verbosity, f: F)
where
    F: Fn(),
{
    if verbosity == Verbosity::Verbose && !forbid_verbose_output {
        f();
    }
}
