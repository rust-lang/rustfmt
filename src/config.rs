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

use lists::{SeparatorTactic, ListTactic};

macro_rules! configuration_option_enum{
    ($e:ident: $( $x:ident ),+ $(,)*) => {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum $e {
            $( $x ),+
        }

        impl_enum_decodable!($e, $( $x ),+);
    }
}

configuration_option_enum! { NewlineStyle:
    Windows, // \r\n
    Unix, // \n
    Native, // \r\n in Windows, \n on other platforms
}

configuration_option_enum! { BraceStyle:
    AlwaysNextLine,
    PreferSameLine,
    // Prefer same line except where there is a where clause, in which case force
    // the brace to the next line.
    SameLineWhere,
}

// How to indent a function's return type.
configuration_option_enum! { ReturnIndent:
    // Aligned with the arguments
    WithArgs,
    // Aligned with the where clause
    WithWhereClause,
}

// How to stle a struct literal.
configuration_option_enum! { StructLitStyle:
    // First line on the same line as the opening brace, all lines aligned with
    // the first line.
    Visual,
    // First line is on a new line and all lines align with block indent.
    Block,
    // FIXME Maybe we should also have an option to align types.
}

configuration_option_enum! { BlockIndentStyle:
    // Same level as parent.
    Inherit,
    // One level deeper than parent.
    Tabbed,
    // Aligned with block open.
    Visual,
}

configuration_option_enum! { Density:
    // Fit as much on one line as possible.
    Compressed,
    // Use more lines.
    Tall,
    // Try to compress if the body is empty.
    CompressedIfEmpty,
}

configuration_option_enum! { TypeDensity:
    // No spaces around "=" and "+"
    Compressed,
    // Spaces around " = " and " + "
    Wide,
}

impl Density {
    pub fn to_list_tactic(self) -> ListTactic {
        match self {
            Density::Compressed => ListTactic::Mixed,
            Density::Tall | Density::CompressedIfEmpty => ListTactic::HorizontalVertical,
        }
    }
}

configuration_option_enum! { LicensePolicy:
    // Do not place license text at top of files
    NoLicense,
    // Use the text in "license" field as the license
    TextLicense,
    // Use a text file as the license text
    FileLicense,
}

configuration_option_enum! { MultilineStyle:
    // Use horizontal layout if it fits in one line, fall back to vertical
    PreferSingle,
    // Use vertical layout
    ForceMulti,
}

impl MultilineStyle {
    pub fn to_list_tactic(self) -> ListTactic {
        match self {
            MultilineStyle::PreferSingle => ListTactic::HorizontalVertical,
            MultilineStyle::ForceMulti => ListTactic::Vertical,
        }
    }
}

configuration_option_enum! { ReportTactic:
    Always,
    Unnumbered,
    Never,
}

configuration_option_enum! { WriteMode:
    // Backsup the original file and overwrites the orignal.
    Replace,
    // Overwrites original file without backup.
    Overwrite,
    // Write the output to stdout.
    Display,
    // Write the diff to stdout.
    Diff,
    // Display how much of the input file was processed
    Coverage,
    // Unfancy stdout
    Plain,
    // Output a checkstyle XML file.
    Checkstyle,
}

// This trait and the following impl blocks are there so that we an use
// UCFS inside the get_docs() function on types for configs.
pub trait ConfigType {
    fn get_variant_names() -> String;
}

impl ConfigType for bool {
    fn get_variant_names() -> String {
        String::from("<boolean>")
    }
}

impl ConfigType for usize {
    fn get_variant_names() -> String {
        String::from("<unsigned integer>")
    }
}

impl ConfigType for String {
    fn get_variant_names() -> String {
        String::from("<string>")
    }
}

pub struct ConfigHelpItem {
    option_name: &'static str,
    doc_string: &'static str,
    variant_names: String,
    default: &'static str,
}

impl ConfigHelpItem {
    pub fn option_name(&self) -> &'static str {
        self.option_name
    }

    pub fn doc_string(&self) -> &'static str {
        self.doc_string
    }

    pub fn variant_names(&self) -> &String {
        &self.variant_names
    }

    pub fn default(&self) -> &'static str {
        self.default
    }
}

macro_rules! create_config {
    ($($i:ident: $ty:ty, $def:expr, $( $dstring:expr ),+ );+ $(;)*) => (
        #[derive(RustcDecodable, Clone)]
        pub struct Config {
            $(pub $i: $ty),+
        }

        /// Equivalent to `Config` except that each field is wrapped in an `Option`.
        ///
        /// This can be decoded into from TOML, and then later merged into a `Config` or another
        /// `PartialConfig`.
        ///
        /// # Examples
        ///
        /// Decode a TOML value into a `PartialConfig`:
        ///
        /// ```ignore
        /// extern crate toml;
        /// use config::{Config, PartialConfig};
        /// let toml_str = r#"
        ///     ideal_width = 72
        /// "#;
        ///
        /// let partial: PartialConfig = toml::decode_str(toml_str);
        /// ```
        ///
        /// Later, merge the `PartialConfig` into the default `Config`:
        ///
        /// ```ignore
        /// # extern crate toml;
        /// # use config::{Config, PartialConfig};
        /// # let toml_str = r#"
        /// #     ideal_width = 72
        /// # "#;
        ///
        /// let partial: PartialConfig = toml::decode_str(toml_str);
        /// let config = Config::Default().merge(partial);
        /// assert_eq!(72, config.ideal_width);
        /// ```
        #[derive(RustcDecodable, Clone)]
        pub struct PartialConfig {
            $(pub $i: Option<$ty>),+
        }

        impl PartialConfig {

            /// Create a `PartialConfig` with all fields set to `None`.
            pub fn new() -> PartialConfig {
                PartialConfig {
                $(
                    $i: None,
                )+
                }

            }

            /// Merge `other` into `self, overwriting fields in `self` with any non-`None` fields
            /// in `other`.
            pub fn merge(&mut self, other: &PartialConfig) -> &mut PartialConfig {
            $(
                if other.$i.is_some() {
                    self.$i = other.$i.clone();
                }
             )+
                self
            }
        }

        impl Default for PartialConfig {
            fn default() -> PartialConfig {
                PartialConfig::new()
            }
        }

        // Convenience impl.
        impl From<WriteMode> for PartialConfig {
            fn from(write_mode: WriteMode) -> PartialConfig {
                PartialConfig {
                    write_mode: Some(write_mode), ..PartialConfig::default()
                }
            }
        }

        // Convenience impl.
        impl From<Option<WriteMode>> for PartialConfig {
            fn from(write_mode: Option<WriteMode>) -> PartialConfig {
                PartialConfig {
                    write_mode: write_mode, ..PartialConfig::default()
                }
            }
        }

        /// Applies settings in `partial` on top of the default `Config`.
        impl From<PartialConfig> for Config {
            fn from(partial: PartialConfig) -> Config {
                Config::default().merge(&partial)
            }
        }

        /// Applies settings in `partial` on top of the default `Config`.
        impl<'a> From<&'a PartialConfig> for Config {
            fn from(partial: &'a PartialConfig) -> Config {
                Config::default().merge(partial)
            }
        }

        impl Config {

            /// Merge `partial` into `self, overwriting fields in `self` with any non-`None` fields
            /// in `partial`.
            pub fn merge(mut self, partial: &PartialConfig) -> Config {
            $(
                if let Some(val) = partial.$i {
                    self.$i = val;
                }
            )+
                self
            }

            pub fn override_value(&mut self, key: &str, val: &str) {
                match key {
                    $(
                        stringify!($i) => {
                            self.$i = val.parse::<$ty>().unwrap();
                        }
                    )+
                    _ => panic!("Bad config key!")
                }
            }

            pub fn print_docs() {
                use std::cmp;
                let max = 0;
                $( let max = cmp::max(max, stringify!($i).len()+1); )+
                let mut space_str = String::with_capacity(max);
                for _ in 0..max {
                    space_str.push(' ');
                }
                println!("Configuration Options:");
                $(
                    let name_raw = stringify!($i);
                    let mut name_out = String::with_capacity(max);
                    for _ in name_raw.len()..max-1 {
                        name_out.push(' ')
                    }
                    name_out.push_str(name_raw);
                    name_out.push(' ');
                    println!("{}{} Default: {:?}",
                             name_out,
                             <$ty>::get_variant_names(),
                             $def);
                    $(
                        println!("{}{}", space_str, $dstring);
                    )+
                    println!("");
                )+
            }
        }

        // Template for the default configuration
        impl Default for Config {
            fn default() -> Config {
                Config {
                    $(
                        $i: $def,
                    )+
                }
            }
        }
    )
}

create_config! {
    verbose: bool, false, "Use verbose output";
    skip_children: bool, false, "Don't reformat out of line modules";
    max_width: usize, 100, "Maximum width of each line";
    ideal_width: usize, 80, "Ideal width of each line";
    tab_spaces: usize, 4, "Number of spaces per tab";
    fn_call_width: usize, 60,
        "Maximum width of the args of a function call before falling back to vertical formatting";
    struct_lit_width: usize, 16,
        "Maximum width in the body of a struct lit before falling back to vertical formatting";
    newline_style: NewlineStyle, NewlineStyle::Unix, "Unix or Windows line endings";
    fn_brace_style: BraceStyle, BraceStyle::SameLineWhere, "Brace style for functions";
    item_brace_style: BraceStyle, BraceStyle::SameLineWhere, "Brace style for structs and enums";
    impl_empty_single_line: bool, true, "Put empty-body implementations on a single line";
    fn_empty_single_line: bool, true, "Put empty-body functions on a single line";
    fn_single_line: bool, false, "Put single-expression functions on a single line";
    fn_return_indent: ReturnIndent, ReturnIndent::WithArgs,
        "Location of return type in function declaration";
    fn_args_paren_newline: bool, true, "If function argument parenthesis goes on a newline";
    fn_args_density: Density, Density::Tall, "Argument density in functions";
    fn_args_layout: StructLitStyle, StructLitStyle::Visual, "Layout of function arguments";
    fn_arg_indent: BlockIndentStyle, BlockIndentStyle::Visual, "Indent on function arguments";
    type_punctuation_density: TypeDensity, TypeDensity::Wide,
        "Determines if '+' or '=' are wrapped in spaces in the punctuation of types";
    // Should we at least try to put the where clause on the same line as the rest of the
    // function decl?
    where_density: Density, Density::CompressedIfEmpty, "Density of a where clause";
    // Visual will be treated like Tabbed
    where_indent: BlockIndentStyle, BlockIndentStyle::Tabbed, "Indentation of a where clause";
    where_layout: ListTactic, ListTactic::Vertical, "Element layout inside a where clause";
    where_pred_indent: BlockIndentStyle, BlockIndentStyle::Visual,
        "Indentation style of a where predicate";
    where_trailing_comma: bool, false, "Put a trailing comma on where clauses";
    generics_indent: BlockIndentStyle, BlockIndentStyle::Visual, "Indentation of generics";
    struct_trailing_comma: SeparatorTactic, SeparatorTactic::Vertical,
        "If there is a trailing comma on structs";
    struct_lit_trailing_comma: SeparatorTactic, SeparatorTactic::Vertical,
        "If there is a trailing comma on literal structs";
    struct_lit_style: StructLitStyle, StructLitStyle::Block, "Style of struct definition";
    struct_lit_multiline_style: MultilineStyle, MultilineStyle::PreferSingle,
        "Multiline style on literal structs";
    enum_trailing_comma: bool, true, "Put a trailing comma on enum declarations";
    report_todo: ReportTactic, ReportTactic::Always,
        "Report all, none or unnumbered occurrences of TODO in source file comments";
    report_fixme: ReportTactic, ReportTactic::Never,
        "Report all, none or unnumbered occurrences of FIXME in source file comments";
    chain_base_indent: BlockIndentStyle, BlockIndentStyle::Visual, "Indent on chain base";
    chain_indent: BlockIndentStyle, BlockIndentStyle::Visual, "Indentation of chain";
    reorder_imports: bool, false, "Reorder import statements alphabetically";
    single_line_if_else: bool, false, "Put else on same line as closing brace for if statements";
    format_strings: bool, true, "Format string literals, or leave as is";
    chains_overflow_last: bool, true, "Allow last call in method chain to break the line";
    take_source_hints: bool, true, "Retain some formatting characteristics from the source code";
    hard_tabs: bool, false, "Use tab characters for indentation, spaces for alignment";
    wrap_comments: bool, false, "Break comments to fit on the line";
    normalise_comments: bool, true, "Convert /* */ comments to // comments where possible";
    wrap_match_arms: bool, true, "Wrap multiline match arms in blocks";
    match_block_trailing_comma: bool, false,
        "Put a trailing comma after a block based match arm (non-block arms are not affected)";
    match_wildcard_trailing_comma: bool, true, "Put a trailing comma after a wildcard arm";
    write_mode: WriteMode, WriteMode::Replace,
        "What Write Mode to use when none is supplied: Replace, Overwrite, Display, Diff, Coverage";
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_merge_overrides() {
        let config = Config::default().merge(&PartialConfig {
            ideal_width: Some(37),
            ..PartialConfig::default()
        });
        assert_eq!(37, config.ideal_width);
    }

    #[test]
    fn test_partial_config_merge_overrides() {
        let mut config = PartialConfig::default();
        config.merge(&PartialConfig { ideal_width: Some(37), ..PartialConfig::default() });
        assert_eq!(Some(37), config.ideal_width);
    }

    #[test]
    fn test_config_merge_does_not_override_if_none() {
        let mut config = Config { ideal_width: 37, ..Config::default() };
        config = config.merge(&PartialConfig::new());
        assert_eq!(37, config.ideal_width);
    }

    #[test]
    fn test_partial_config_merge_does_not_override_if_none() {
        let mut config = PartialConfig { ideal_width: Some(37), ..PartialConfig::default() };
        config.merge(&PartialConfig::new());
        assert_eq!(Some(37), config.ideal_width);
    }
}
