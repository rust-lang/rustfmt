use std::default::Default;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::{env, fs};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::config_type::ConfigType;
#[allow(unreachable_pub)]
pub use crate::config::file_lines::{FileLines, FileName, Range};
#[allow(unreachable_pub)]
pub use crate::config::lists::*;
#[allow(unreachable_pub)]
pub use crate::config::options::*;
use crate::is_nightly_channel;

use rustfmt_config_proc_macro::rustfmt_config;

#[macro_use]
pub(crate) mod config_type;
#[macro_use]
pub(crate) mod options;

pub(crate) mod file_lines;
pub(crate) mod license;
pub(crate) mod lists;

fn rustfmt_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

// This macro defines configuration options used in rustfmt. Each option
// is defined as follows:
//
// `name: value type, default value, is stable, description;`
#[rustfmt_config]
#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Maximum width of each line.
    #[rustfmt::config(default(100), stable("1.0.0"), setter(set_max_width_inner))]
    max_width: usize,
    /// Use tab characters for indentation, spaces for alignment.
    #[rustfmt::config(default(false), stable("1.0.0"))]
    hard_tabs: bool,
    /// Number of spaces per tab.
    #[rustfmt::config(default(4), stable("1.0.0"))]
    tab_spaces: usize,
    /// Unix or Windows line endings.
    #[rustfmt::config(default(NewlineStyle::Auto), stable("1.0.0"))]
    newline_style: NewlineStyle,
    /// Whether to use different formatting for items and expressions
    /// if they satisfy a heuristic notion of 'small'.
    #[rustfmt::config(
        default(Heuristics::Default),
        stable("1.0.0"),
        setter(set_heuristics_inner)
    )]
    use_small_heuristics: Heuristics,
    /// How do we indent expressions or items.
    #[rustfmt::config(default(IndentStyle::Block), stable("2.0.0"))]
    indent_style: IndentStyle,

    // Comments. macros, and strings
    /// Break comments to fit on the line.
    #[rustfmt::config(default(false))]
    wrap_comments: bool,
    /// Format the code snippet in doc comments.
    #[rustfmt::config(default(false))]
    format_code_in_doc_comments: bool,
    /// Maximum length of comments. No effect unless `wrap_comments` is set to `true`.
    #[rustfmt::config(default(80))]
    comment_width: usize,
    /// Convert /* */ comments to // comments where possible.
    #[rustfmt::config(default(false))]
    normalize_comments: bool,
    /// Normalize doc attributes as doc comments.
    #[rustfmt::config(default(false))]
    normalize_doc_attributes: bool,
    /// Beginning of file must match license template.
    #[rustfmt::config(default(String::default()))]
    license_template_path: String,
    /// Format string literals where necessary.
    #[rustfmt::config(default(false))]
    format_strings: bool,
    /// Format the metavariable matching patterns in macros.
    #[rustfmt::config(default(false))]
    format_macro_matchers: bool,
    /// Format the bodies of macros.
    #[rustfmt::config(default(true))]
    format_macro_bodies: bool,

    // Single line expressions and items
    /// Put empty-body functions and impls on a single line.
    #[rustfmt::config(default(true))]
    empty_item_single_line: bool,
    /// Put small struct literals on a single line.
    #[rustfmt::config(default(true))]
    struct_lit_single_line: bool,
    /// Put single-expression functions on a single line.
    #[rustfmt::config(default(false))]
    fn_single_line: bool,
    /// Force where-clauses to be on a single line.
    #[rustfmt::config(default(false))]
    where_single_line: bool,

    // Imports
    /// Indent of imports.
    #[rustfmt::config(default(IndentStyle::Block))]
    imports_indent: IndentStyle,
    /// Item layout inside a import block.
    #[rustfmt::config(default(ListTactic::Mixed))]
    imports_layout: ListTactic,
    /// Merge imports.
    #[rustfmt::config(default(false))]
    merge_imports: bool,

    // Ordering
    /// Reorder import and extern crate statements alphabetically.
    #[rustfmt::config(default(true))]
    reorder_imports: bool,
    /// Reorder module statements alphabetically in group.
    #[rustfmt::config(default(true))]
    reorder_modules: bool,
    /// Reorder impl items.
    #[rustfmt::config(default(false))]
    reorder_impl_items: bool,

    // Spaces around punctuation
    /// Determines if '+' or '=' are wrapped in spaces in the punctuation of types.
    #[rustfmt::config(default(TypeDensity::Wide))]
    type_punctuation_density: TypeDensity,
    /// Leave a space before the colon.
    #[rustfmt::config(default(false))]
    space_before_colon: bool,
    /// Leave a space after the colon.
    #[rustfmt::config(default(true))]
    space_after_colon: bool,
    /// Put spaces around the  .. and ..= range operators.
    #[rustfmt::config(default(false))]
    spaces_around_ranges: bool,
    /// Where to put a binary operator when a binary expression goes multiline.
    #[rustfmt::config(default(SeparatorPlace::Front))]
    binop_separator: SeparatorPlace,

    // Misc.
    /// Remove nested parens.
    #[rustfmt::config(default(true))]
    remove_nested_parens: bool,
    /// Combine control expressions with function calls.
    #[rustfmt::config(default(true))]
    combine_control_expr: bool,
    /// Allow trailing bracket/brace delimited expressions to overflow.
    #[rustfmt::config(default(false))]
    overflow_delimited_expr: bool,
    /// Align struct fields if their diffs fits within threshold.
    #[rustfmt::config(default(0))]
    struct_field_align_threshold: usize,
    /// Align enum variants discrims, if their diffs fit within threshold.
    #[rustfmt::config(default(0))]
    enum_discrim_align_threshold: usize,
    /// Wrap the body of arms in blocks when it does not fit on the same line
    /// with the pattern of arms.
    #[rustfmt::config(default(true))]
    match_arm_blocks: bool,
    /// Force multiline closure bodies and match arms to be wrapped in a block.
    #[rustfmt::config(default(false))]
    force_multiline_blocks: bool,
    /// Control the layout of arguments in a function.
    #[rustfmt::config(default(Density::Tall))]
    fn_args_layout: Density,
    /// Brace style for items.
    #[rustfmt::config(default(BraceStyle::SameLineWhere))]
    brace_style: BraceStyle,
    #[rustfmt::config(default(ControlBraceStyle::AlwaysSameLine))]
    /// Brace style for control flow constructs.
    control_brace_style: ControlBraceStyle,
    /// Add trailing semicolon after break, continue and return.
    #[rustfmt::config(default(true))]
    trailing_semicolon: bool,
    /// How to handle trailing commas for lists.
    #[rustfmt::config(default(SeparatorTactic::Vertical))]
    trailing_comma: SeparatorTactic,
    /// Put a trailing comma after a block based match arm (non-block arms are not affected).
    #[rustfmt::config(default(false))]
    match_block_trailing_comma: bool,
    /// Maximum number of blank lines which can be put between items.
    #[rustfmt::config(default(1))]
    blank_lines_upper_bound: usize,
    /// Minimum number of blank lines which must be put between items.
    #[rustfmt::config(default(0))]
    blank_lines_lower_bound: usize,
    /// The edition of the parser (RFC 2052).
    #[rustfmt::config(default(Edition::Edition2015), stable("1.0.0"))]
    edition: Edition,
    /// Version of formatting rules.
    #[rustfmt::config(default(Version::One))]
    version: Version,
    #[rustfmt::config(default(0))]
    /// Write an item and its attribute on the same line if
    /// their combined width is below a threshold.
    inline_attribute_width: usize,

    // Options that can change the source code beyond whitespace/blocks (somewhat linty things)
    /// Merge multiple `#[derive(...)]` into a single one.
    #[rustfmt::config(default(true), stable("1.0.0"))]
    merge_derives: bool,
    /// Replace uses of the try! macro by the ? shorthand.
    #[rustfmt::config(default(false), stable("1.0.0"))]
    use_try_shorthand: bool,
    /// Use field initialization shorthand if possible.
    #[rustfmt::config(default(false), stable("1.0.0"))]
    use_field_init_shorthand: bool,
    /// Always print the abi for extern items.
    #[rustfmt::config(default(true), stable("1.0.0"))]
    force_explicit_abi: bool,
    /// Replace strings of _ wildcards by a single `..` in tuple patterns.
    #[rustfmt::config(default(false))]
    condense_wildcard_suffixes: bool,

    // Control options (changes the operation of rustfmt, rather than the formatting)
    /// What Color option to use when none is supplied: Always, Never, Auto.
    #[rustfmt::config(default(Color::Auto))]
    color: Color,
    /// Require a specific version of rustfmt.
    #[rustfmt::config(default(rustfmt_version()))]
    required_version: String,
    /// Enables unstable features. Only available on nightly channel.
    #[rustfmt::config(default(false))]
    unstable_features: bool,
    /// Don't reformat anything.
    #[rustfmt::config(default(false))]
    disable_all_formatting: bool,
    /// Don't reformat out of line modules.
    #[rustfmt::config(default(false))]
    skip_children: bool,
    /// Hide errors from the parser.
    #[rustfmt::config(default(false))]
    hide_parse_errors: bool,
    /// Error if unable to get all lines within max_width.
    #[rustfmt::config(default(false))]
    error_on_line_overflow: bool,
    /// Error if unable to get comments or string literals within max_width,
    /// or they are left with trailing whitespaces
    #[rustfmt::config(default(false))]
    error_on_unformatted: bool,
    /// Report all, none or unnumbered occurrences of TODO in source file comments.
    #[rustfmt::config(default(ReportTactic::Never))]
    report_todo: ReportTactic,
    /// Report all, none or unnumbered occurrences of FIXME in source file comments.
    #[rustfmt::config(default(ReportTactic::Never))]
    report_fixme: ReportTactic,
    /// Skip formatting the specified files and directories.
    #[rustfmt::config(default(IgnoreList::default()))]
    ignore: IgnoreList,

    // Not user-facing
    /// How much to information to emit to the user.
    #[serde(skip)]
    pub(crate) verbose: Verbosity,
    /// Lines to format; this is not supported in rustfmt.toml, and can only be specified
    /// via the --file-lines option.
    #[serde(skip)]
    pub(crate) file_lines: FileLines,
    /// 'small' heuristic values.
    #[serde(skip)]
    pub(crate) width_heuristics: WidthHeuristics,
    /// What emit Mode to use when none is supplied.
    #[serde(skip)]
    pub(crate) emit_mode: EmitMode,
    /// Backup changed files.
    #[serde(skip)]
    pub(crate) make_backup: bool,
    /// Prints the names of mismatched files that were formatted. Prints the names of
    /// files that would be formated when used with `--check` mode.
    #[serde(skip)]
    pub(crate) print_misformatted_file_names: bool,

    #[serde(skip)]
    pub license_template: Option<Regex>,
}

impl Config {
    pub fn license_template(&self) -> Option<&Regex> {
        self.license_template.as_ref()
    }

    pub fn print_misformatted_file_names(&self) -> bool {
        self.print_misformatted_file_names
    }

    pub fn set_print_misformatted_file_names(&mut self, val: bool) {
        self.print_misformatted_file_names = val;
    }

    pub fn make_backup(&self) -> bool {
        self.make_backup
    }

    pub fn set_make_backup(&mut self, val: bool) {
        self.make_backup = val;
    }

    pub fn emit_mode(&self) -> EmitMode {
        self.emit_mode.clone()
    }

    pub fn set_emit_mode(&mut self, val: EmitMode) {
        self.emit_mode = val;
    }

    pub fn width_heuristics(&self) -> &WidthHeuristics {
        &self.width_heuristics
    }

    pub fn set_width_heuristics(&mut self, val: WidthHeuristics) {
        self.width_heuristics = val;
    }

    pub fn file_lines(&self) -> &FileLines {
        &self.file_lines
    }

    pub fn set_file_lines(&mut self, val: FileLines) {
        self.file_lines = val;
    }

    pub fn verbose(&self) -> Verbosity {
        self.verbose.clone()
    }

    pub fn set_verbose(&mut self, val: Verbosity) {
        self.verbose = val;
    }

    pub fn to_toml(&self) -> Result<String, ConfigError> {
        toml::to_string(self).map_err(ConfigError::TomlSerializationError)
    }

    fn set_max_width_inner(&mut self, val: usize) {
        self.max_width = Some(val);
        self.set_heuristics();
    }

    fn set_heuristics_inner(&mut self, val: Heuristics) {
        self.use_small_heuristics = Some(val);
        self.set_heuristics();
    }

    fn set_heuristics(&mut self) {
        if self.use_small_heuristics() == Heuristics::Default {
            let max_width = self.max_width();
            self.width_heuristics = WidthHeuristics::scaled(max_width);
        } else if self.use_small_heuristics() == Heuristics::Max {
            let max_width = self.max_width();
            self.width_heuristics = WidthHeuristics::set(max_width);
        } else {
            self.width_heuristics = WidthHeuristics::null();
        }
    }

    fn set_license_template(&mut self) {
        if self.is_license_template_path_set() {
            let lt_path = self.license_template_path();
            if lt_path.len() > 0 {
                match license::load_and_compile_template(&lt_path) {
                    Ok(re) => self.license_template = Some(re),
                    Err(msg) => {
                        eprintln!("Warning for license template file {:?}: {}", lt_path, msg)
                    }
                }
            }
        }
    }

    fn add_ignore_prefix(&mut self, dir: &Path) {
        self.ignore.as_mut().map(|ignore| ignore.add_prefix(dir));
    }

    pub(crate) fn version_meets_requirement(&self) -> bool {
        if self.is_required_version_set() {
            let version = env!("CARGO_PKG_VERSION");
            let required_version = self.required_version();
            if version != required_version {
                println!(
                    "Error: rustfmt version ({}) doesn't match the required version ({})",
                    version, required_version,
                );
                return false;
            }
        }

        true
    }

    /// Constructs a `Config` from the toml file specified at `file_path`.
    ///
    /// This method only looks at the provided path, for a method that
    /// searches parents for a `rustfmt.toml` see `from_resolved_toml_path`.
    ///
    /// Returns a `Config` if the config could be read and parsed from
    /// the file, otherwise errors.
    pub(super) fn from_toml_path(file_path: &Path) -> Result<Config, Error> {
        let mut file = File::open(&file_path)?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        Config::from_toml(&toml, file_path.parent().unwrap())
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))
    }

    /// Resolves the config for input in `dir`.
    ///
    /// Searches for `rustfmt.toml` beginning with `dir`, and
    /// recursively checking parents of `dir` if no config file is found.
    /// If no config file exists in `dir` or in any parent, a
    /// default `Config` will be returned (and the returned path will be empty).
    ///
    /// Returns the `Config` to use, and the path of the project file if there was
    /// one.
    pub(super) fn from_resolved_toml_path(dir: &Path) -> Result<(Config, Option<PathBuf>), Error> {
        /// Try to find a project file in the given directory and its parents.
        /// Returns the path of a the nearest project file if one exists,
        /// or `None` if no project file was found.
        fn resolve_project_file(dir: &Path) -> Result<Option<PathBuf>, Error> {
            let mut current = if dir.is_relative() {
                env::current_dir()?.join(dir)
            } else {
                dir.to_path_buf()
            };

            current = fs::canonicalize(current)?;

            loop {
                match get_toml_path(&current) {
                    Ok(Some(path)) => return Ok(Some(path)),
                    Err(e) => return Err(e),
                    _ => (),
                }

                // If the current directory has no parent, we're done searching.
                if !current.pop() {
                    break;
                }
            }

            // If nothing was found, check in the home directory.
            if let Some(home_dir) = dirs::home_dir() {
                if let Some(path) = get_toml_path(&home_dir)? {
                    return Ok(Some(path));
                }
            }

            // If none was found ther either, check in the user's configuration directory.
            if let Some(mut config_dir) = dirs::config_dir() {
                config_dir.push("rustfmt");
                if let Some(path) = get_toml_path(&config_dir)? {
                    return Ok(Some(path));
                }
            }

            Ok(None)
        }

        match resolve_project_file(dir)? {
            None => Ok((Config::default(), None)),
            Some(path) => Config::from_toml_path(&path).map(|config| (config, Some(path))),
        }
    }

    pub(crate) fn from_toml(toml: &str, dir: &Path) -> Result<Config, String> {
        let parsed: ::toml::Value = toml
            .parse()
            .map_err(|e| format!("Could not parse TOML: {}", e))?;
        let mut err = String::new();
        let table = parsed
            .as_table()
            .ok_or_else(|| String::from("Parsed config was not table"))?;
        for key in table.keys() {
            if !Config::is_valid_name(key) {
                let msg = &format!("Warning: Unknown configuration option `{}`\n", key);
                err.push_str(msg)
            }
        }
        match parsed.try_into::<Config>() {
            Ok(mut parsed_config) => {
                parsed_config.validate().map_err(|e| e.to_string())?;
                if !err.is_empty() {
                    eprint!("{}", err);
                }
                parsed_config.add_ignore_prefix(dir);
                parsed_config.set_license_template();
                Ok(parsed_config)
            }
            Err(e) => {
                err.push_str("Error: Decoding config file failed:\n");
                err.push_str(format!("{}\n", e).as_str());
                err.push_str("Please check your config file.");
                Err(err)
            }
        }
    }
}

/// Loads a config by checking the client-supplied options and if appropriate, the
/// file system (including searching the file system for overrides).
pub fn load_config<O: CliOptions>(
    file_path: Option<&Path>,
    options: Option<O>,
) -> Result<(Config, Option<PathBuf>), Error> {
    let over_ride = match options {
        Some(ref opts) => config_path(opts)?,
        None => None,
    };

    let result = if let Some(over_ride) = over_ride {
        Config::from_toml_path(over_ride.as_ref()).map(|p| (p, Some(over_ride.to_owned())))
    } else if let Some(file_path) = file_path {
        Config::from_resolved_toml_path(file_path)
    } else {
        Ok((Config::default(), None))
    };

    result.map(|(mut c, p)| {
        if let Some(options) = options {
            options.apply_to(&mut c);
        }
        (c, p)
    })
}

// Check for the presence of known config file names (`rustfmt.toml, `.rustfmt.toml`) in `dir`
//
// Return the path if a config file exists, empty if no file exists, and Error for IO errors
fn get_toml_path(dir: &Path) -> Result<Option<PathBuf>, Error> {
    const CONFIG_FILE_NAMES: [&str; 2] = [".rustfmt.toml", "rustfmt.toml"];
    for config_file_name in &CONFIG_FILE_NAMES {
        let config_file = dir.join(config_file_name);
        match fs::metadata(&config_file) {
            // Only return if it's a file to handle the unlikely situation of a directory named
            // `rustfmt.toml`.
            Ok(ref md) if md.is_file() => return Ok(Some(config_file)),
            // Return the error if it's something other than `NotFound`; otherwise we didn't
            // find the project file yet, and continue searching.
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    return Err(e);
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn config_path(options: &dyn CliOptions) -> Result<Option<PathBuf>, Error> {
    let config_path_not_found = |path: &str| -> Result<Option<PathBuf>, Error> {
        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Error: unable to find a config file for the given path: `{}`",
                path
            ),
        ))
    };

    // Read the config_path and convert to parent dir if a file is provided.
    // If a config file cannot be found from the given path, return error.
    match options.config_path() {
        Some(path) if !path.exists() => config_path_not_found(path.to_str().unwrap()),
        Some(path) if path.is_dir() => {
            let config_file_path = get_toml_path(path)?;
            if config_file_path.is_some() {
                Ok(config_file_path)
            } else {
                config_path_not_found(path.to_str().unwrap())
            }
        }
        path => Ok(path.map(ToOwned::to_owned)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str;

    const STABLE_OPTION_NAME: &str = "max_width";
    const UNSTABLE_OPTION_NAME: &str = "blank_lines_lower_bound";

    #[test]
    fn test_config_set() {
        let mut config = Config::default();
        config.set_verbose(Verbosity::Quiet);
        assert_eq!(config.verbose(), Verbosity::Quiet);
        config.set_verbose(Verbosity::Normal);
        assert_eq!(config.verbose(), Verbosity::Normal);
    }

    #[test]
    fn test_was_set() {
        let config = Config::from_toml("hard_tabs = true", Path::new("")).unwrap();

        assert_eq!(config.is_hard_tabs_set(), true);
    }

    #[test]
    fn test_print_docs_exclude_unstable() {
        use self::Config;

        let mut output = Vec::new();
        Config::print_docs(&mut output, false).unwrap();

        let s = str::from_utf8(&output).unwrap();

        assert_eq!(s.contains(STABLE_OPTION_NAME), true, "\n{}", s);
        assert_eq!(s.contains(UNSTABLE_OPTION_NAME), false, "\n{}", s);
        assert_eq!(s.contains("(unstable)"), false, "\n{}", s);
    }

    #[test]
    fn test_print_docs_include_unstable() {
        use self::Config;

        let mut output = Vec::new();
        Config::print_docs(&mut output, true).unwrap();

        let s = str::from_utf8(&output).unwrap();
        assert_eq!(s.contains(STABLE_OPTION_NAME), true, "\n{}", s);
        assert_eq!(s.contains(UNSTABLE_OPTION_NAME), true, "\n{}", s);
        assert_eq!(s.contains("(unstable)"), true, "\n{}", s);
    }

    #[test]
    fn test_empty_string_license_template_path() {
        let toml = r#"license_template_path = """#;
        let config = Config::from_toml(toml, Path::new("")).unwrap();
        assert!(config.license_template.is_none());
    }

    #[test]
    fn test_valid_license_template_path() {
        let toml = r#"license_template_path = "tests/license-template/lt.txt""#;
        let config = Config::from_toml(toml, Path::new("")).unwrap();
        assert!(config.license_template.is_some());
    }

    #[test]
    fn test_dump_default_config() {
        let default_config = format!(
            r#"max_width = 100
hard_tabs = false
tab_spaces = 4
newline_style = "Auto"
use_small_heuristics = "Default"
indent_style = "Block"
wrap_comments = false
format_code_in_doc_comments = false
comment_width = 80
normalize_comments = false
normalize_doc_attributes = false
license_template_path = ""
format_strings = false
format_macro_matchers = false
format_macro_bodies = true
empty_item_single_line = true
struct_lit_single_line = true
fn_single_line = false
where_single_line = false
imports_indent = "Block"
imports_layout = "Mixed"
merge_imports = false
reorder_imports = true
reorder_modules = true
reorder_impl_items = false
type_punctuation_density = "Wide"
space_before_colon = false
space_after_colon = true
spaces_around_ranges = false
binop_separator = "Front"
remove_nested_parens = true
combine_control_expr = true
overflow_delimited_expr = false
struct_field_align_threshold = 0
enum_discrim_align_threshold = 0
match_arm_blocks = true
force_multiline_blocks = false
fn_args_layout = "Tall"
brace_style = "SameLineWhere"
control_brace_style = "AlwaysSameLine"
trailing_semicolon = true
trailing_comma = "Vertical"
match_block_trailing_comma = false
blank_lines_upper_bound = 1
blank_lines_lower_bound = 0
edition = "2015"
version = "One"
inline_attribute_width = 0
merge_derives = true
use_try_shorthand = false
use_field_init_shorthand = false
force_explicit_abi = true
condense_wildcard_suffixes = false
color = "Auto"
required_version = "{}"
unstable_features = false
disable_all_formatting = false
skip_children = false
hide_parse_errors = false
error_on_line_overflow = false
error_on_unformatted = false
report_todo = "Never"
report_fixme = "Never"
ignore = []
"#,
            env!("CARGO_PKG_VERSION")
        );
        let toml = Config::all_options().to_toml().unwrap();
        assert_eq!(&toml, &default_config);
    }

    // FIXME(#2183): these tests cannot be run in parallel because they use env vars.
    // #[test]
    // fn test_as_not_nightly_channel() {
    //     let mut config = Config::default();
    //     assert_eq!(config.was_set().unstable_features(), false);
    //     config.set().unstable_features(true);
    //     assert_eq!(config.was_set().unstable_features(), false);
    // }

    // #[test]
    // fn test_as_nightly_channel() {
    //     let v = ::std::env::var("CFG_RELEASE_CHANNEL").unwrap_or(String::from(""));
    //     ::std::env::set_var("CFG_RELEASE_CHANNEL", "nightly");
    //     let mut config = Config::default();
    //     config.set().unstable_features(true);
    //     assert_eq!(config.was_set().unstable_features(), false);
    //     config.set().unstable_features(true);
    //     assert_eq!(config.unstable_features(), true);
    //     ::std::env::set_var("CFG_RELEASE_CHANNEL", v);
    // }

    // #[test]
    // fn test_unstable_from_toml() {
    //     let mut config = Config::from_toml("unstable_features = true").unwrap();
    //     assert_eq!(config.was_set().unstable_features(), false);
    //     let v = ::std::env::var("CFG_RELEASE_CHANNEL").unwrap_or(String::from(""));
    //     ::std::env::set_var("CFG_RELEASE_CHANNEL", "nightly");
    //     config = Config::from_toml("unstable_features = true").unwrap();
    //     assert_eq!(config.was_set().unstable_features(), true);
    //     assert_eq!(config.unstable_features(), true);
    //     ::std::env::set_var("CFG_RELEASE_CHANNEL", v);
    // }
}
