use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use syntax::ast;
use syntax::errors::emitter::{ColorConfig, Emitter, EmitterWriter};
use syntax::errors::{Diagnostic, Handler};
use syntax::parse::ParseSess as RawParseSess;
use syntax::source_map::{FilePathMapping, SourceMap};

use crate::ignore_path::IgnorePathSet;
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
            self.source_map(),
        )
    }

    pub(crate) fn is_file_parsed(&self, path: &Path) -> bool {
        self.source_map()
            .get_source_file(&syntax_pos::FileName::Real(path.to_path_buf()))
            .is_some()
    }

    pub(super) fn can_reset_errors(&self) -> bool {
        *self.can_reset_errors.borrow()
    }

    pub(crate) fn ignore_file(&self, path: &FileName) -> bool {
        self.ignore_path_set.as_ref().is_match(&path)
    }

    pub(crate) fn set_silent_emitter(&mut self) {
        self.parse_sess.span_diagnostic = Handler::with_emitter(true, None, silent_emitter());
    }

    pub(crate) fn inner(&self) -> &RawParseSess {
        &self.parse_sess
    }

    pub(crate) fn has_errors(&self) -> bool {
        self.parse_sess.span_diagnostic.has_errors()
    }

    pub(crate) fn reset_errors(&self) {
        self.parse_sess.span_diagnostic.reset_err_count();
    }

    pub(crate) fn source_map(&self) -> &SourceMap {
        &self.parse_sess.source_map()
    }

    pub(crate) fn emit_diagnostics(&self, diagnostics: Vec<Diagnostic>) {
        for diagnostic in diagnostics {
            self.parse_sess.span_diagnostic.emit_diagnostic(&diagnostic);
        }
    }
}
