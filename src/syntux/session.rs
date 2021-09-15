use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use rustc_data_structures::sync::{Lrc, Send};
use rustc_errors::emitter::{Emitter, EmitterWriter};
use rustc_errors::{ColorConfig, Diagnostic, Handler, Level as DiagnosticLevel};
use rustc_session::parse::ParseSess as RawParseSess;
use rustc_span::{
    source_map::{FilePathMapping, SourceMap},
    symbol, BytePos, Span,
};

use crate::config::file_lines::LineRange;
use crate::ignore_path::IgnorePathSet;
use crate::source_map::LineRangeUtils;
use crate::utils::starts_with_newline;
use crate::visitor::SnippetProvider;
use crate::{Config, ErrorKind, FileName};

/// ParseSess holds structs necessary for constructing a parser.
pub(crate) struct ParseSess {
    parse_sess: RawParseSess,
    ignore_path_set: Lrc<IgnorePathSet>,
    can_reset_errors: Lrc<AtomicBool>,
}

/// Emitter which discards every error.
struct SilentEmitter;

impl Emitter for SilentEmitter {
    fn source_map(&self) -> Option<&Lrc<SourceMap>> {
        None
    }
    fn emit_diagnostic(&mut self, _db: &Diagnostic) {}
}

fn silent_emitter() -> Box<dyn Emitter + Send> {
    Box::new(SilentEmitter {})
}

/// Emit errors against every files expect ones specified in the `ignore_path_set`.
struct SilentOnIgnoredFilesEmitter {
    ignore_path_set: Lrc<IgnorePathSet>,
    source_map: Lrc<SourceMap>,
    emitter: Box<dyn Emitter + Send>,
    has_non_ignorable_parser_errors: bool,
    can_reset: Lrc<AtomicBool>,
}

impl SilentOnIgnoredFilesEmitter {
    fn handle_non_ignoreable_error(&mut self, db: &Diagnostic) {
        self.has_non_ignorable_parser_errors = true;
        self.can_reset.store(false, Ordering::Release);
        self.emitter.emit_diagnostic(db);
    }
}

impl Emitter for SilentOnIgnoredFilesEmitter {
    fn source_map(&self) -> Option<&Lrc<SourceMap>> {
        None
    }
    fn emit_diagnostic(&mut self, db: &Diagnostic) {
        if db.level == DiagnosticLevel::Fatal {
            return self.handle_non_ignoreable_error(db);
        }
        if let Some(primary_span) = &db.span.primary_span() {
            let file_name = self.source_map.span_to_filename(*primary_span);
            if let rustc_span::FileName::Real(rustc_span::RealFileName::LocalPath(ref path)) =
                file_name
            {
                if self
                    .ignore_path_set
                    .is_match(&FileName::Real(path.to_path_buf()))
                {
                    if !self.has_non_ignorable_parser_errors {
                        self.can_reset.store(true, Ordering::Release);
                    }
                    return;
                }
            };
        }
        self.handle_non_ignoreable_error(db);
    }
}

fn default_handler(
    source_map: Lrc<SourceMap>,
    ignore_path_set: Lrc<IgnorePathSet>,
    can_reset: Lrc<AtomicBool>,
    hide_parse_errors: bool,
) -> Handler {
    let supports_color = term::stderr().map_or(false, |term| term.supports_color());
    let color_cfg = if supports_color {
        ColorConfig::Auto
    } else {
        ColorConfig::Never
    };

    let emitter = if hide_parse_errors {
        silent_emitter()
    } else {
        Box::new(EmitterWriter::stderr(
            color_cfg,
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
    pub(crate) fn new(config: &Config) -> Result<ParseSess, ErrorKind> {
        let ignore_path_set = match IgnorePathSet::from_ignore_list(&config.ignore()) {
            Ok(ignore_path_set) => Lrc::new(ignore_path_set),
            Err(e) => return Err(ErrorKind::InvalidGlobPattern(e)),
        };
        let source_map = Lrc::new(SourceMap::new(FilePathMapping::empty()));
        let can_reset_errors = Lrc::new(AtomicBool::new(false));

        let handler = default_handler(
            Lrc::clone(&source_map),
            Lrc::clone(&ignore_path_set),
            Lrc::clone(&can_reset_errors),
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
                rustc_span::RealFileName::LocalPath(path.to_path_buf()),
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

    pub(crate) fn span_to_file_contents(&self, span: Span) -> Lrc<rustc_span::SourceFile> {
        self.parse_sess
            .source_map()
            .lookup_source_file(span.data().lo)
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

    pub(crate) fn line_of_byte_pos(&self, pos: BytePos) -> usize {
        self.parse_sess.source_map().lookup_char_pos(pos).line
    }

    pub(crate) fn span_to_debug_info(&self, span: Span) -> String {
        self.parse_sess.source_map().span_to_diagnostic_string(span)
    }

    pub(crate) fn inner(&self) -> &RawParseSess {
        &self.parse_sess
    }

    pub(crate) fn snippet_provider(&self, span: Span) -> SnippetProvider {
        let source_file = self.parse_sess.source_map().lookup_char_pos(span.lo()).file;
        SnippetProvider::new(
            source_file.start_pos,
            source_file.end_pos,
            Lrc::clone(source_file.src.as_ref().unwrap()),
        )
    }

    pub(crate) fn get_original_snippet(&self, file_name: &FileName) -> Option<Lrc<String>> {
        self.parse_sess
            .source_map()
            .get_source_file(&file_name.into())
            .and_then(|source_file| source_file.src.clone())
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
        self.can_reset_errors.load(Ordering::Acquire)
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
        use crate::is_nightly_channel;
        use crate::utils::mk_sp;
        use rustc_span::{FileName as SourceMapFileName, MultiSpan, RealFileName, DUMMY_SP};
        use std::path::PathBuf;
        use std::sync::atomic::AtomicU32;

        struct TestEmitter {
            num_emitted_errors: Lrc<AtomicU32>,
        }

        impl Emitter for TestEmitter {
            fn source_map(&self) -> Option<&Lrc<SourceMap>> {
                None
            }
            fn emit_diagnostic(&mut self, _db: &Diagnostic) {
                self.num_emitted_errors.fetch_add(1, Ordering::Release);
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
            num_emitted_errors: Lrc<AtomicU32>,
            can_reset: Lrc<AtomicBool>,
            source_map: Option<Lrc<SourceMap>>,
            ignore_list: Option<IgnoreList>,
        ) -> SilentOnIgnoredFilesEmitter {
            let emitter_writer = TestEmitter { num_emitted_errors };
            let source_map =
                source_map.unwrap_or_else(|| Lrc::new(SourceMap::new(FilePathMapping::empty())));
            let ignore_path_set = Lrc::new(
                IgnorePathSet::from_ignore_list(&ignore_list.unwrap_or_default()).unwrap(),
            );
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
            let num_emitted_errors = Lrc::new(AtomicU32::new(0));
            let can_reset_errors = Lrc::new(AtomicBool::new(false));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let source_map = Lrc::new(SourceMap::new(FilePathMapping::empty()));
            let source =
                String::from(r#"extern "system" fn jni_symbol!( funcName ) ( ... ) -> {} "#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Lrc::clone(&num_emitted_errors),
                Lrc::clone(&can_reset_errors),
                Some(Lrc::clone(&source_map)),
                Some(ignore_list),
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let fatal_diagnostic = build_diagnostic(DiagnosticLevel::Fatal, Some(span));
            emitter.emit_diagnostic(&fatal_diagnostic);
            assert_eq!(num_emitted_errors.load(Ordering::Acquire), 1);
            assert_eq!(can_reset_errors.load(Ordering::Acquire), false);
        }

        #[test]
        fn handles_recoverable_parse_error_in_ignored_file() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Lrc::new(AtomicU32::new(0));
            let can_reset_errors = Lrc::new(AtomicBool::new(false));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let source_map = Lrc::new(SourceMap::new(FilePathMapping::empty()));
            let source = String::from(r#"pub fn bar() { 1x; }"#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Lrc::clone(&num_emitted_errors),
                Lrc::clone(&can_reset_errors),
                Some(Lrc::clone(&source_map)),
                Some(ignore_list),
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let non_fatal_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(span));
            emitter.emit_diagnostic(&non_fatal_diagnostic);
            assert_eq!(num_emitted_errors.load(Ordering::Acquire), 0);
            assert_eq!(can_reset_errors.load(Ordering::Acquire), true);
        }

        #[test]
        fn handles_recoverable_parse_error_in_non_ignored_file() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Lrc::new(AtomicU32::new(0));
            let can_reset_errors = Lrc::new(AtomicBool::new(false));
            let source_map = Lrc::new(SourceMap::new(FilePathMapping::empty()));
            let source = String::from(r#"pub fn bar() { 1x; }"#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("foo.rs"))),
                source,
            );
            let mut emitter = build_emitter(
                Lrc::clone(&num_emitted_errors),
                Lrc::clone(&can_reset_errors),
                Some(Lrc::clone(&source_map)),
                None,
            );
            let span = MultiSpan::from_span(mk_sp(BytePos(0), BytePos(1)));
            let non_fatal_diagnostic = build_diagnostic(DiagnosticLevel::Warning, Some(span));
            emitter.emit_diagnostic(&non_fatal_diagnostic);
            assert_eq!(num_emitted_errors.load(Ordering::Acquire), 1);
            assert_eq!(can_reset_errors.load(Ordering::Acquire), false);
        }

        #[test]
        fn handles_mix_of_recoverable_parse_error() {
            if !is_nightly_channel!() {
                return;
            }
            let num_emitted_errors = Lrc::new(AtomicU32::new(0));
            let can_reset_errors = Lrc::new(AtomicBool::new(false));
            let source_map = Lrc::new(SourceMap::new(FilePathMapping::empty()));
            let ignore_list = get_ignore_list(r#"ignore = ["foo.rs"]"#);
            let bar_source = String::from(r#"pub fn bar() { 1x; }"#);
            let foo_source = String::from(r#"pub fn foo() { 1x; }"#);
            let fatal_source =
                String::from(r#"extern "system" fn jni_symbol!( funcName ) ( ... ) -> {} "#);
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("bar.rs"))),
                bar_source,
            );
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("foo.rs"))),
                foo_source,
            );
            source_map.new_source_file(
                SourceMapFileName::Real(RealFileName::LocalPath(PathBuf::from("fatal.rs"))),
                fatal_source,
            );
            let mut emitter = build_emitter(
                Lrc::clone(&num_emitted_errors),
                Lrc::clone(&can_reset_errors),
                Some(Lrc::clone(&source_map)),
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
            assert_eq!(num_emitted_errors.load(Ordering::Acquire), 2);
            assert_eq!(can_reset_errors.load(Ordering::Acquire), false);
        }
    }
}
