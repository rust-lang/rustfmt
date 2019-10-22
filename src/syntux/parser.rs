use std::panic::{catch_unwind, AssertUnwindSafe};

use syntax::ast;
use syntax::errors::Diagnostic;
use syntax::parse::parser::Parser as RawParser;
use syntax::parse::DirectoryOwnership;
use syntax::source_map::DUMMY_SP;

use crate::syntux::session::ParseSess;
use crate::{Config, Input};

/// A parser for Rust source code.
pub(crate) struct Parser<'a> {
    parser: RawParser<'a>,
    sess: &'a ParseSess,
}

/// A builder for the `Parser`.
#[derive(Default)]
pub(crate) struct ParserBuilder<'a> {
    config: Option<&'a Config>,
    sess: Option<&'a ParseSess>,
    input: Option<Input>,
    directory_ownership: Option<DirectoryOwnership>,
}

impl<'a> ParserBuilder<'a> {
    pub(crate) fn input(mut self, input: Input) -> ParserBuilder<'a> {
        self.input = Some(input);
        self
    }

    pub(crate) fn sess(mut self, sess: &'a ParseSess) -> ParserBuilder<'a> {
        self.sess = Some(sess);
        self
    }

    pub(crate) fn config(mut self, config: &'a Config) -> ParserBuilder<'a> {
        self.config = Some(config);
        self
    }

    pub(crate) fn directory_ownership(
        mut self,
        directory_ownership: Option<DirectoryOwnership>,
    ) -> ParserBuilder<'a> {
        self.directory_ownership = directory_ownership;
        self
    }

    pub(crate) fn build(self) -> Result<Parser<'a>, ParserError> {
        let config = self.config.ok_or(ParserError::NoConfig)?;
        let sess = self.sess.ok_or(ParserError::NoParseSess)?;
        let input = self.input.ok_or(ParserError::NoInput)?;

        let mut parser = match Self::parser(sess.inner(), input, self.directory_ownership) {
            Ok(p) => p,
            Err(db) => {
                sess.emit_diagnostics(db);
                // report.add_parsing_error();
                return Err(ParserError::ParserCreationError);
            }
        };

        parser.cfg_mods = false;

        if config.skip_children() {
            parser.recurse_into_file_modules = false;
        }

        Ok(Parser { parser, sess })
    }

    fn parser(
        sess: &'a syntax::parse::ParseSess,
        input: Input,
        directory_ownership: Option<DirectoryOwnership>,
    ) -> Result<syntax::parse::parser::Parser<'a>, Vec<Diagnostic>> {
        match input {
            Input::File(ref file) => Ok(if let Some(directory_ownership) = directory_ownership {
                syntax::parse::new_sub_parser_from_file(
                    sess,
                    file,
                    directory_ownership,
                    None,
                    DUMMY_SP,
                )
            } else {
                syntax::parse::new_parser_from_file(sess, file)
            }),
            Input::Text(text) => syntax::parse::maybe_new_parser_from_source_str(
                sess,
                syntax::source_map::FileName::Custom("stdin".to_owned()),
                text,
            )
            .map(|mut parser| {
                parser.recurse_into_file_modules = false;
                parser
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParserError {
    NoConfig,
    NoParseSess,
    NoInput,
    ParserCreationError,
    ParseError,
    ParsePanicError,
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_crate(
        config: &'a Config,
        input: Input,
        directory_ownership: Option<DirectoryOwnership>,
        sess: &'a ParseSess,
    ) -> Result<ast::Crate, ParserError> {
        let mut parser = ParserBuilder::default()
            .config(config)
            .input(input)
            .directory_ownership(directory_ownership)
            .sess(sess)
            .build()?;

        parser.parse_crate_inner()
    }

    fn parse_crate_inner(&mut self) -> Result<ast::Crate, ParserError> {
        let mut parser = AssertUnwindSafe(&mut self.parser);

        match catch_unwind(move || parser.parse_crate_mod()) {
            Ok(Ok(krate)) => {
                if !self.sess.has_errors() {
                    return Ok(krate);
                }

                if self.sess.can_reset_errors() {
                    self.sess.reset_errors();
                    return Ok(krate);
                }

                Err(ParserError::ParseError)
            }
            Ok(Err(mut db)) => {
                db.emit();
                Err(ParserError::ParseError)
            }
            Err(_) => Err(ParserError::ParsePanicError),
        }
    }
}
