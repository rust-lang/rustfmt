use std::collections::{hash_set, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

use rustfmt_config_proc_macro::config_type;
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::config::lists::*;
use crate::config::Config;

#[config_type]
pub enum NewlineStyle {
    /// Auto-detect based on the raw source input.
    Auto,
    /// Force CRLF (`\r\n`).
    Windows,
    /// Force CR (`\n).
    Unix,
    /// `\r\n` in Windows, `\n`` on other platforms.
    Native,
}

impl Default for NewlineStyle {
    fn default() -> Self {
        NewlineStyle::Auto
    }
}

#[config_type]
/// Where to put the opening brace of items (`fn`, `impl`, etc.).
pub enum BraceStyle {
    /// Put the opening brace on the next line.
    AlwaysNextLine,
    /// Put the opening brace on the same line, if possible.
    PreferSameLine,
    /// Prefer the same line except where there is a where-clause, in which
    /// case force the brace to be put on the next line.
    SameLineWhere,
}

#[config_type]
/// Where to put the opening brace of conditional expressions (`if`, `match`, etc.).
pub enum ControlBraceStyle {
    /// K&R style, Rust community default
    AlwaysSameLine,
    /// Stroustrup style
    ClosingNextLine,
    /// Allman style
    AlwaysNextLine,
}

#[config_type]
/// How to indent.
pub enum IndentStyle {
    /// First line on the same line as the opening brace, all lines aligned with
    /// the first line.
    Visual,
    /// First line is on a new line and all lines align with **block** indent.
    Block,
}

#[config_type]
/// How to place a list-like items.
/// FIXME: Issue-3581: this should be renamed to ItemsLayout when publishing 2.0
pub enum Density {
    /// Fit as much on one line as possible.
    Compressed,
    /// Items are placed horizontally if sufficient space, vertically otherwise.
    Tall,
    /// Place every item on a separate line.
    Vertical,
}

#[config_type]
/// Spacing around type combinators.
pub enum TypeDensity {
    /// No spaces around "=" and "+"
    Compressed,
    /// Spaces around " = " and " + "
    Wide,
}

#[config_type]
/// Heuristic settings that can be used to simply
/// the configuration of the granular width configurations
/// like `struct_lit_width`, `array_width`, etc.
pub enum Heuristics {
    /// Turn off any heuristics
    Off,
    /// Turn on max heuristics
    Max,
    /// Use scaled values based on the value of `max_width`
    Scaled,
}

impl Density {
    pub fn to_list_tactic(self, len: usize) -> ListTactic {
        match self {
            Density::Compressed => ListTactic::Mixed,
            Density::Tall => ListTactic::HorizontalVertical,
            Density::Vertical if len == 1 => ListTactic::Horizontal,
            Density::Vertical => ListTactic::Vertical,
        }
    }
}

#[config_type]
pub enum ReportTactic {
    Always,
    Unnumbered,
    Never,
}

#[config_type]
/// rustfmt format style version.
pub enum Version {
    /// 1.x.y. When specified, rustfmt will format in the same style as 1.0.0.
    One,
    /// 2.x.y. When specified, rustfmt will format in the the latest style.
    Two,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WidthHeuristics {
    // Maximum width of the args of a function call before falling back
    // to vertical formatting.
    pub fn_call_width: usize,
    // Maximum width of the args of a function-like attributes before falling
    // back to vertical formatting.
    pub attr_fn_like_width: usize,
    // Maximum width in the body of a struct lit before falling back to
    // vertical formatting.
    pub struct_lit_width: usize,
    // Maximum width in the body of a struct variant before falling back
    // to vertical formatting.
    pub struct_variant_width: usize,
    // Maximum width of an array literal before falling back to vertical
    // formatting.
    pub array_width: usize,
    // Maximum length of a chain to fit on a single line.
    pub chain_width: usize,
    // Maximum line length for single line if-else expressions. A value
    // of zero means always break if-else expressions.
    pub single_line_if_else_max_width: usize,
}

impl fmt::Display for WidthHeuristics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl WidthHeuristics {
    // Using this WidthHeuristics means we ignore heuristics.
    pub fn null() -> WidthHeuristics {
        WidthHeuristics {
            fn_call_width: usize::max_value(),
            attr_fn_like_width: usize::max_value(),
            struct_lit_width: 0,
            struct_variant_width: 0,
            array_width: usize::max_value(),
            chain_width: usize::max_value(),
            single_line_if_else_max_width: 0,
        }
    }

    pub fn set(max_width: usize) -> WidthHeuristics {
        WidthHeuristics {
            fn_call_width: max_width,
            attr_fn_like_width: max_width,
            struct_lit_width: max_width,
            struct_variant_width: max_width,
            array_width: max_width,
            chain_width: max_width,
            single_line_if_else_max_width: max_width,
        }
    }

    // scale the default WidthHeuristics according to max_width
    pub fn scaled(max_width: usize) -> WidthHeuristics {
        const DEFAULT_MAX_WIDTH: usize = 100;
        let max_width_ratio = if max_width > DEFAULT_MAX_WIDTH {
            let ratio = max_width as f32 / DEFAULT_MAX_WIDTH as f32;
            // round to the closest 0.1
            (ratio * 10.0).round() / 10.0
        } else {
            1.0
        };
        WidthHeuristics {
            fn_call_width: (60.0 * max_width_ratio).round() as usize,
            attr_fn_like_width: (70.0 * max_width_ratio).round() as usize,
            struct_lit_width: (18.0 * max_width_ratio).round() as usize,
            struct_variant_width: (35.0 * max_width_ratio).round() as usize,
            array_width: (60.0 * max_width_ratio).round() as usize,
            chain_width: (60.0 * max_width_ratio).round() as usize,
            single_line_if_else_max_width: (50.0 * max_width_ratio).round() as usize,
        }
    }
}

/// A set of directories, files and modules that rustfmt should ignore.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct IgnoreList {
    /// A set of path specified in rustfmt.toml.
    path_set: HashSet<PathBuf>,
    /// A path to rustfmt.toml.
    rustfmt_toml_path: PathBuf,
}

impl fmt::Display for IgnoreList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.path_set.iter()).finish()
    }
}

impl Serialize for IgnoreList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.path_set.len()))?;
        for e in &self.path_set {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for IgnoreList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HashSetVisitor;
        impl<'v> Visitor<'v> for HashSetVisitor {
            type Value = HashSet<PathBuf>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a sequence of path")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'v>,
            {
                let mut path_set = HashSet::new();
                while let Some(elem) = seq.next_element()? {
                    path_set.insert(elem);
                }
                Ok(path_set)
            }
        }
        Ok(IgnoreList {
            path_set: deserializer.deserialize_seq(HashSetVisitor)?,
            rustfmt_toml_path: PathBuf::new(),
        })
    }
}

impl<'a> IntoIterator for &'a IgnoreList {
    type Item = &'a PathBuf;
    type IntoIter = hash_set::Iter<'a, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.path_set.iter()
    }
}

impl IgnoreList {
    pub fn add_prefix(&mut self, dir: &Path) {
        self.rustfmt_toml_path = dir.to_path_buf();
    }

    pub fn rustfmt_toml_path(&self) -> &Path {
        &self.rustfmt_toml_path
    }

    /// Merges `self` into `other`, returning a new `IgnoreList`. The resulting `IgnoreList` uses
    /// the `rustfmt_toml_path` of `other`, and only contains paths that are in `other`'s
    /// `rustfmt_toml_path`.
    pub fn merge_into(self, other: Self) -> Self {
        let old_rustfmt_toml_path = self.rustfmt_toml_path;
        let new_rustfmt_toml_path = other.rustfmt_toml_path;
        let relocalized_old_paths: HashSet<PathBuf> = self
            .path_set
            .into_iter()
            .map(|p| old_rustfmt_toml_path.parent().unwrap().join(p))
            .map(|p| {
                // Only keep old paths that are also in the new config path
                p.strip_prefix(new_rustfmt_toml_path.parent().unwrap())
                    .map(PathBuf::from)
            })
            .flatten()
            .collect();

        let mut path_set = other.path_set;
        path_set.extend(relocalized_old_paths);

        Self {
            path_set,
            rustfmt_toml_path: new_rustfmt_toml_path,
        }
    }
}

impl std::str::FromStr for IgnoreList {
    type Err = &'static str;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err("IgnoreList is not parsable")
    }
}

/// Maps client-supplied options to Rustfmt's internals, mostly overriding
/// values in a config with values from the command line.
pub trait CliOptions {
    fn apply_to(&self, config: &mut Config);
    fn config_path(&self) -> Option<&Path>;
}

/// The edition of the syntax and semantics of code (RFC 2052).
#[config_type]
pub enum Edition {
    #[value = "2015"]
    #[doc_hint = "2015"]
    /// Edition 2015.
    Edition2015,
    #[value = "2018"]
    #[doc_hint = "2018"]
    /// Edition 2018.
    Edition2018,
}

impl Default for Edition {
    fn default() -> Self {
        Self::Edition2018
    }
}

impl From<Edition> for rustc_span::edition::Edition {
    fn from(edition: Edition) -> Self {
        match edition {
            Edition::Edition2015 => Self::Edition2015,
            Edition::Edition2018 => Self::Edition2018,
        }
    }
}

/// Controls how rustfmt should handle leading pipes on match arms.
#[config_type]
pub enum MatchArmLeadingPipe {
    /// Place leading pipes on all match arms
    Always,
    /// Never emit leading pipes on match arms
    Never,
    /// Preserve any existing leading pipes
    Preserve,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::config::IgnoreList;

    #[test]
    fn test_ignore_list_merge_into() {
        let ignore_list_outer = IgnoreList {
            path_set: vec!["b/c/d.rs", "b/c/d/e.rs", "b/other.rs"]
                .into_iter()
                .map(PathBuf::from)
                .collect(),
            rustfmt_toml_path: PathBuf::from("a/rustfmt.toml"),
        };

        let ignore_list_inner = IgnoreList {
            path_set: vec!["f.rs", "g/h.rs"]
                .into_iter()
                .map(PathBuf::from)
                .collect(),
            rustfmt_toml_path: PathBuf::from("a/b/c/rustfmt.toml"),
        };

        let ignore_list_merged = ignore_list_outer.merge_into(ignore_list_inner);

        assert_eq!(
            ignore_list_merged.rustfmt_toml_path,
            PathBuf::from("a/b/c/rustfmt.toml")
        );
        assert_eq!(
            ignore_list_merged.path_set,
            vec!["d.rs", "d/e.rs", "f.rs", "g/h.rs"]
                .into_iter()
                .map(PathBuf::from)
                .collect()
        );
    }
}
