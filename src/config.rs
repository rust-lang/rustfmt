// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate toml;

use lists::SeparatorTactic;
use issues::ReportTactic;


#[derive(RustcDecodable, Clone)]
pub struct Config {
    pub max_width: usize,
    pub ideal_width: usize,
    pub leeway: usize,
    pub tab_spaces: usize,
    pub newline_style: NewlineStyle,
    pub fn_brace_style: BraceStyle,
    pub fn_return_indent: ReturnIndent,
    pub fn_args_paren_newline: bool,
    pub struct_trailing_comma: SeparatorTactic,
    pub struct_lit_trailing_comma: SeparatorTactic,
    pub struct_lit_style: StructLitStyle,
    pub enum_trailing_comma: bool,
    pub report_todo: ReportTactic,
    pub report_fixme: ReportTactic,
    pub reorder_imports: bool, // Alphabetically, case sensitive.
    pub features: Vec<Feature>,
}

impl Config {
    pub fn from_toml(toml: &str) -> Config {
        let parsed = toml.parse().unwrap();
        match toml::decode(parsed) {
            Some(decoded) => decoded,
            None => {
                println!("Decoding config file failed. Config:\n{}", toml);
                let parsed: toml::Value = toml.parse().unwrap();
                println!("\n\nParsed:\n{:?}", parsed);
                panic!();
            }
        }
    }

    pub fn feature(&self, f: Feature) -> bool {
        self.features.len() == 0 || self.features.contains(&f)
    }
}



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NewlineStyle {
    Windows, // \r\n
    Unix, // \n
}

impl_enum_decodable!(NewlineStyle, Windows, Unix);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BraceStyle {
    AlwaysNextLine,
    PreferSameLine,
    // Prefer same line except where there is a where clause, in which case force
    // the brace to the next line.
    SameLineWhere,
}

impl_enum_decodable!(BraceStyle, AlwaysNextLine, PreferSameLine, SameLineWhere);

// How to indent a function's return type.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ReturnIndent {
    // Aligned with the arguments
    WithArgs,
    // Aligned with the where clause
    WithWhereClause,
}

impl_enum_decodable!(ReturnIndent, WithArgs, WithWhereClause);

// Which features to run.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Feature {
    // Check for overlong lines, trailing whitespace, TODOs, etc.
    Tidy,
    // Trim trailing whitespace.
    Trim,

    FnDecls,
    // Also covers statements and blocks.
    Expressions,
    // Also covers trait and impl items (and imports).
    // FIXME would be good to split out imports.
    Items,

    Comments,
}

impl_enum_decodable!(Feature, Tidy, Trim, FnDecls, Expressions, Items, Comments);
