use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use syntax::ast;
use syntax::errors::emitter::{ColorConfig, Emitter, EmitterWriter};
use syntax::errors::{Diagnostic, Handler};
use syntax::parse::ParseSess as RawParseSess;
use syntax::source_map::{FilePathMapping, SourceMap};
use syntax_pos::{BytePos, Span};

use crate::config::file_lines::LineRange;
use crate::ignore_path::IgnorePathSet;
use crate::source_map::LineRangeUtils;
use crate::utils::starts_with_newline;
use crate::visitor::SnippetProvider;
use crate::{Config, ErrorKind, FileName};

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
}

fn silent_emitter() -> Box<SilentEmitter> {
    Box::new(SilentEmitter {})
}

/// Emit errors against every files expect ones specified in the `ignore_path_set`.
struct SilentOnIgnoredFilesEmitter {
    ignore_path_set: Rc<IgnorePathSet>,
    source_map: Rc<SourceMap>,
    emitter: EmitterWriter,
    has_non_ignorable_parser_errors: bool,
    can_reset: Rc<RefCell<bool>>,
}

impl Emitter for SilentOnIgnoredFilesEmitter {
    fn emit_diagnostic(&mut self, db: &Diagnostic) {
        if let Some(primary_span) = &db.span.primary_span() {
            let file_name = self.source_map.span_to_filename(*primary_span);
            match file_name {
                syntax_pos::FileName::Real(ref path) => {
                    if self
                        .ignore_path_set
                        .is_match(&FileName::Real(path.to_path_buf()))
                    {
                        if !self.has_non_ignorable_parser_errors {
                            *self.can_reset.borrow_mut() = true;
                        }
                        return;
                    }
                }
                _ => (),
            };
        }

        self.has_non_ignorable_parser_errors = true;
        *self.can_reset.borrow_mut() = false;
        self.emitter.emit_diagnostic(db);
    }
}

fn silent_handler() -> Handler {
    Handler::with_emitter(true, None, silent_emitter())
}

fn default_handler(
    source_map: Rc<SourceMap>,
    ignore_path_set: Rc<IgnorePathSet>,
    can_reset: Rc<RefCell<bool>>,
) -> Handler {
    let supports_color = term::stderr().map_or(false, |term| term.supports_color());
    let color_cfg = if supports_color {
        ColorConfig::Auto
    } else {
        ColorConfig::Never
    };

    let emitter = EmitterWriter::stderr(
        color_cfg,
        Some(source_map.clone()),
        false,
        false,
        None,
        false,
    );
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
            Ok(ignore_path_set) => Rc::new(ignore_path_set),
            Err(e) => return Err(ErrorKind::InvalidGlobPattern(e)),
        };
        let source_map = Rc::new(SourceMap::new(FilePathMapping::empty()));
        let can_reset_errors = Rc::new(RefCell::new(false));

        let handler = if config.hide_parse_errors() {
            silent_handler()
        } else {
            default_handler(
                Rc::clone(&source_map),
                Rc::clone(&ignore_path_set),
                Rc::clone(&can_reset_errors),
            )
        };
        let parse_sess = RawParseSess::with_span_handler(handler, source_map);

        Ok(ParseSess {
            parse_sess,
            ignore_path_set,
            can_reset_errors,
        })
    }

    pub(crate) fn default_submod_path(
        &self,
        id: ast::Ident,
        relative: Option<ast::Ident>,
        dir_path: &Path,
    ) -> syntax::parse::parser::ModulePath {
        syntax::parse::parser::Parser::default_submod_path(
            id,
            relative,
            dir_path,
            self.parse_sess.source_map(),
        )
    }

    pub(crate) fn is_file_parsed(&self, path: &Path) -> bool {
        self.parse_sess
            .source_map()
            .get_source_file(&syntax_pos::FileName::Real(path.to_path_buf()))
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

        let first_line_str = match file_lines {
            Some(fl) => fl
                .file
                .get_line(fl.lines[0].line_index)
                .map(|s| s.into_owned()),
            None => return String::new(),
        };

        first_line_str.unwrap_or(String::new())
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

    pub(crate) fn get_original_snippet(&self, file_name: &FileName) -> Option<Rc<String>> {
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
