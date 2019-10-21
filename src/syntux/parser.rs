use syntax::parse::parser::Parser as RawParser;

use crate::Input;

/// A parser for Rust source code.
pub struct Parser<'a> {
    parser: RawParser<'a>,
}

/// A builder for the `Parser`.
pub struct ParserBuilder {
    silent: bool,
    input: Input,
}

impl ParserBuilder {
    pub fn input(&mut self, input: Input) -> &mut ParserBuilder {
        self.input = input;
        self
    }

    pub fn build(self) -> Result<Parser, ParserError> {
        match self.input {
            Input::File(ref file) => {
                parse::new_parser_from_file(par)
            }
        }
        Ok(Parser)
    }
}

enum ParserError {
    ParserCreationError,
}

impl<'a> Parser<'a> {
    pub fn from_string(text: String) -> Result<Parser<'a>, ParserError> {
    }
}