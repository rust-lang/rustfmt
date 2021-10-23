use std::cell::RefCell;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;

use rustc_data_structures::sync::{Lrc, Send};
use rustc_errors::emitter::{Emitter, EmitterWriter};
use rustc_errors::{ColorConfig, Diagnostic, Handler, Level as DiagnosticLevel};
use rustc_session::parse::ParseSess as RawParseSess;
use rustc_span::{
    source_map::{FilePathMapping, SourceMap},
    symbol, BytePos, Span,
};

use crate::config::{file_lines::LineRange, Config, FileName};
use crate::formatting::{
    source_map::LineRangeUtils, utils::starts_with_newline, visitor::SnippetProvider,
};
use crate::result::OperationError;
use ignore_path::IgnorePathSet;

mod ignore_path;

/// ParseSess holds structs necessary for constructing a parser.
pub(crate) struct ParseSess {
    parse_sess: RawParseSess,
    ignore_path_set: Rc<IgnorePathSet>,
    can_reset_errors: Rc<RefCell<bool>>,
}

/// Emitter which discards every error.
struct SilentEmitter;

impl Emitter for SilentEmitter {
    fn emit_diagnostic(&mut self, _db: &Diagnostic) {}
    fn source_map(&self) -> Option<&Lrc<SourceMap>> {
        None
    }
}

fn silent_emitter() -> Box<dyn Emitter + Send> {
    Box::new(SilentEmitter {})
}

/// Emit errors against every files expect ones specified in the `ignore_path_set`.
struct SilentOnIgnoredFilesEmitter {
    ignore_path_set: Rc<IgnorePathSet>,
    source_map: Rc<SourceMap>,
    emitter: Box<dyn Emitter + Send>,
    has_non_ignorable_parser_errors: bool,
    can_reset: Rc<RefCell<bool>>,
}

impl SilentOnIgnoredFilesEmitter {
    fn handle_non_ignoreable_error(&mut self, db: &Diagnostic) {
        self.has_non_ignorable_parser_errors = true;
        *self.can_reset.borrow_mut() = false;
        self.emitter.emit_diagnostic(db);
    }
}

impl Emitter for SilentOnIgnoredFilesEmitter {
    fn emit_diagnostic(&mut self, db: &Diagnostic) {
        if db.level == DiagnosticLevel::Fatal {
            return self.handle_non_ignoreable_error(db);
        }
        if let Some(primary_span) = &db.span.primary_span() {
            let file_name = self.source_map.span_to_filename(*primary_span);
            if let rustc_span::FileName::Real(rustc_span::RealFileName::Named(ref path)) = file_name
            {
                if self
                    .ignore_path_set
                    .is_match(&FileName::Real(path.to_path_buf()))
                {
                    if !self.has_non_ignorable_parser_errors {
                        *self.can_reset.borrow_mut() = true;
                    }
                    return;
                }
            };
        }
        self.handle_non_ignoreable_error(db);
    }

    fn source_map(&self) -> Option<&Lrc<SourceMap>> {
        None
    }
}

fn default_handler(
    source_map: Rc<SourceMap>,
    ignore_path_set: Rc<IgnorePathSet>,
    can_reset: Rc<RefCell<bool>>,
    hide_parse_errors: bool,
) -> Handler {
    let emitter = if hide_parse_errors {
        silent_emitter()
    } else {
        Box::new(EmitterWriter::stderr(
            ColorConfig::Auto,
            Some(source_map.clone()),
            false,
            false,
            None,
            false,
        ))
    };
    Handler::with_emitter(
        true,
        None,
        Box::new(SilentOnIgnoredFilesEmitter {
            has_non_ignorable_parser_errors: false,
            source_map,
            emitter,
            ignore_path_set,
            can_reset,
        }),
    )
}

impl ParseSess {
    pub(crate) fn new(config: &Config) -> Result<ParseSess, OperationError> {
        let ignore_path_set = match IgnorePathSet::from_ignore_list(&config.ignore()) {
            Ok(ignore_path_set) => Rc::new(ignore_path_set),
            Err(e) => return Err(OperationError::InvalidGlobPattern(e)),
        };
        let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
        let can_reset_errors = Rc::new(RefCell::new(false));

        let handler = default_handler(
            Rc::clone(&source_map),
            Rc::clone(&ignore_path_set),
            Rc::clone(&can_reset_errors),
            config.hide_parse_errors(),
        );
        let parse_sess = RawParseSess::with_span_handler(handler, source_map);

        Ok(ParseSess {
            parse_sess,
            ignore_path_set,
            can_reset_errors,
        })
    }

    pub(crate) fn default_submod_path(
        &self,
        id: symbol::Ident,
        relative: Option<symbol::Ident>,
        dir_path: &Path,
    ) -> Result<rustc_expand::module::ModulePathSuccess, rustc_expand::module::ModError<'_>> {
        rustc_expand::module::default_submod_path(&self.parse_sess, id, relative, dir_path)
    }

    pub(crate) fn is_file_parsed(&self, path: &Path) -> bool {
        self.parse_sess
            .source_map()
            .get_source_file(&rustc_span::FileName::Real(
                rustc_span::RealFileName::Named(path.to_path_buf()),
            ))
            .is_some()
    }

    pub(crate) fn ignore_file(&self, path: &FileName) -> bool {
        self.ignore_path_set.as_ref().is_match(&path)
    }

    pub(crate) fn set_silent_emitter(&mut self) {
        self.parse_sess.span_diagnostic = Handler::with_emitter(true, None, silent_emitter());
    }

    pub(crate) fn span_to_filename(&self, span: Span) -> FileName {
        self.parse_sess.source_map().span_to_filename(span).into()
    }

    pub(crate) fn span_to_first_line_string(&self, span: Span) -> String {
        let file_lines = self.parse_sess.source_map().span_to_lines(span).ok();

        match file_lines {
            Some(fl) => fl
                .file
                .get_line(fl.lines[0].line_index)
                .map_or_else(String::new, |s| s.to_string()),
            None => String::new(),
        }
    }

    pub(crate) fn line_bounds(&self, pos: BytePos) -> Option<Range<BytePos>> {
        let line = self.parse_sess.source_map().lookup_line(pos).ok();

        match line {
            Some(line_info) => Some(line_info.sf.line_bounds(line_info.line)),
            None => None,
        }
    }

    pub(crate) fn line_of_byte_pos(&self, pos: BytePos) -> usize {
        self.parse_sess.source_map().lookup_char_pos(pos).line
    }

    pub(crate) fn span_to_debug_info(&self, span: Span) -> String {
        self.parse_sess.source_map().span_to_string(span)
    }

    pub(crate) fn inner(&self) -> &RawParseSess {
        &self.parse_sess
    }

    pub(crate) fn snippet_provider(&self, span: Span) -> SnippetProvider {
        let source_file = self.parse_sess.source_map().lookup_char_pos(span.lo()).file;
        SnippetProvider::new(
            source_file.start_pos,
            source_file.end_pos,
            Rc::clone(source_file.src.as_ref().unwrap()),
        )
    }
}

// Methods that should be restricted within the syntux module.
impl ParseSess {
    pub(super) fn emit_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        for diagnostic in diagnostics {
            self.parse_sess.span_diagnostic.emit_diagnostic(&diagnostic);
        }
    }

    pub(crate) fn emit_or_cancel_diagnostic(&self, diagnostic: &mut Diagnostic) {
        self.parse_sess.span_diagnostic.emit_diagnostic(diagnostic);
        // The Handler will check whether the diagnostic should be emitted
        // based on the user's rustfmt configuration and the originating file
        // that caused the parser error. If the Handler determined it should skip
        // emission then we need to ensure the diagnostic is cancelled.
        if !diagnostic.cancelled() {
            diagnostic.cancel();
        }
    }

    pub(super) fn can_reset_errors(&self) -> bool {
        *self.can_reset_errors.borrow()
    }

    pub(super) fn has_errors(&self) -> bool {
        self.parse_sess.span_diagnostic.has_errors()
    }

    pub(super) fn reset_errors(&self) {
        self.parse_sess.span_diagnostic.reset_err_count();
    }
}

impl LineRangeUtils for ParseSess {
    fn lookup_line_range(&self, span: Span) -> LineRange {
        let snippet = self
            .parse_sess
            .source_map()
            .span_to_snippet(span)
            .unwrap_or_default();
        let lo = self.parse_sess.source_map().lookup_line(span.lo()).unwrap();
        let hi = self.parse_sess.source_map().lookup_line(span.hi()).unwrap();

        debug_assert_eq!(
            lo.sf.name, hi.sf.name,
            "span crossed file boundary: lo: {:?}, hi: {:?}",
            lo, hi
        );

        // in case the span starts with a newline, the line range is off by 1 without the
        // adjustment below
        let offset = 1 + if starts_with_newline(&snippet) { 1 } else { 0 };
        // Line numbers start at 1
        LineRange {
            file: lo.sf.clone(),
            lo: lo.line + offset,
            hi: hi.line + offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod emitter {
        use super::*;
        use crate::config::IgnoreList;
        use crate::formatting::utils::mk_sp;
        use crate::is_nightly_channel;
        use rustc_span::{FileName as SourceMapFileName, MultiSpan, RealFileName, DUMMY_SP};
        use std::path::PathBuf;

        struct TestEmitter {
            num_emitted_errors: Rc<RefCell<u32>>,
        }

        impl Emitter for TestEmitter {
            fn source_map(&self) -> Option<&Lrc<SourceMap>> {
                None
            }
            fn emit_diagnostic(&mut self, _db: &Diagnostic) {
                *self.num_emitted_errors.borrow_mut() += 1;
            }
        }

        fn build_diagnostic(level: DiagnosticLevel, span: Option<MultiSpan>) -> Diagnostic {
            Diagnostic {
                level,
                code: None,
                message: vec![],
                children: vec![],
                suggestions: vec![],
                span: span.unwrap_or_else(MultiSpan::new),
                sort_span: DUMMY_SP,
            }
        }

        fn build_emitter(
            num_emitted_errors: Rc<RefCell<u32>>,
            can_reset: Rc<RefCell<bool>>,
            source_map: Option<Rc<SourceMap>>,
            ignore_list: Option<IgnoreList>,
        ) -> SilentOnIgnoredFilesEmitter {
            let emitter_writer = TestEmitter { num_emitted_errors };
            let source_map =
                source_map.unwrap_or_else(|| Rc::new(SourceMap::new(FilePathMapping::empty())));
            let ignore_path_set =
                Rc::new(IgnorePathSet::from_ignore_list(&ignore_list.unwrap_or_default()).unwrap());
            SilentOnIgnoredFilesEmitter {
                has_non_ignorable_parser_errors: false,
                source_map,
                emitter: Box::new(emitter_writer),
                ignore_path_set,
                can_reset,
            }
        }

        fn get_ignore_list(config: &str) -> IgnoreList {
            Config::from_toml(config, Path::new("")).unwrap().ignore()
        }

        #[test]
        fn handles_fatal_parse_error_in_ignored_file() {
            let num_emitted_errors = Rc::new(RefCell::new(0));
            let can_reset_errors = Rc::new(RefCell::new(false));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
            let source =
                String::from(r#"extern "system" fn jni_symbol!( funcName ) ( ... ) -> {} "#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Rc::clone(&num_emitted_errors),
                Rc::clone(&can_reset_errors),
                Some(Rc::clone(&source_map)),
                Some(ignore_list),
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let fatal_diagnostic = build_diagnostic(DiagnosticLevel::Fatal, Some(span));
            emitter.emit_diagnostic(&fatal_diagnostic);
            assert_eq!(*num_emitted_errors.borrow(), 1);
            assert_eq!(*can_reset_errors.borrow(), false);
        }

        #[test]
        fn handles_recoverable_parse_error_in_ignored_file() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Rc::new(RefCell::new(0));
            let can_reset_errors = Rc::new(RefCell::new(false));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
            let source = String::from(r#"pub fn bar() { 1x; }"#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Rc::clone(&num_emitted_errors),
                Rc::clone(&can_reset_errors),
                Some(Rc::clone(&source_map)),
                Some(ignore_list),
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let non_fatal_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(span));
            emitter.emit_diagnostic(&non_fatal_diagnostic);
            assert_eq!(*num_emitted_errors.borrow(), 0);
            assert_eq!(*can_reset_errors.borrow(), true);
        }

        #[test]
        fn handles_recoverable_parse_error_in_non_ignored_file() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Rc::new(RefCell::new(0));
            let can_reset_errors = Rc::new(RefCell::new(false));
            let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
            let source = String::from(r#"pub fn bar() { 1x; }"#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Rc::clone(&num_emitted_errors),
                Rc::clone(&can_reset_errors),
                Some(Rc::clone(&source_map)),
                None,
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let non_fatal_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(span));
            emitter.emit_diagnostic(&non_fatal_diagnostic);
            assert_eq!(*num_emitted_errors.borrow(), 1);
            assert_eq!(*can_reset_errors.borrow(), false);
        }

        #[test]
        fn handles_mix_of_recoverable_parse_error() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Rc::new(RefCell::new(0));
            let can_reset_errors = Rc::new(RefCell::new(false));
            let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let bar_source = String::from(r#"pub fn bar() { 1x; }"#);
            let foo_source = String::from(r#"pub fn foo() { 1x; }"#);
            let fatal_source =
                String::from(r#"extern "system" fn jni_symbol!( funcName ) ( ... ) -> {} "#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("bar.rs"))),
                bar_source,
            );
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("foo.rs"))),
                foo_source,
            );
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::Named(PathBuf::from("fatal.rs"))),
                fatal_source,
            );
            let mut emitter = build_emitter(
                Rc::clone(&num_emitted_errors),
                Rc::clone(&can_reset_errors),
                Some(Rc::clone(&source_map)),
                Some(ignore_list),
            );
            let bar_span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let foo_span = MultiSpan::from_span(mk_sp(BytePos(21), BytePos(22)));
            let bar_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(bar_span));
            let foo_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(foo_span));
            let fatal_diagnostic = build_diagnostic(DiagnosticLevel::Fatal, None);
            emitter.emit_diagnostic(&bar_diagnostic);
            emitter.emit_diagnostic(&foo_diagnostic);
            emitter.emit_diagnostic(&fatal_diagnostic);
            assert_eq!(*num_emitted_errors.borrow(), 2);
            assert_eq!(*can_reset_errors.borrow(), false);
        }
    }
}
