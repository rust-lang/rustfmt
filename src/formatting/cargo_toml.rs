use itertools::Itertools;
use std::cmp::Ordering;
use toml_edit::{
    visit_mut::*, Decor, Document, Formatted, Item, KeyMut, RawString, Table, TableLike, TomlError,
    Value,
};

use crate::{Config, ErrorKind};

/// Format `Cargo.toml` according to [the Style Guide].
///
/// [the Style Guide]: https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/cargo.md
pub(crate) fn format_cargo_toml_inner(content: &str, config: &Config) -> Result<String, ErrorKind> {
    let mut doc = content.parse::<Document>()?;
    let rules = [
        &mut SortSection {
            current_position: 0,
        } as &mut dyn VisitMut,
        &mut BlankLine { trimming: true },
        &mut KeyValue,
        &mut MultiLine,
        &mut WrapArray {
            max_width: config.max_width(),
        },
        &mut FormatInlineTable {
            max_width: config.max_width(),
            long_tables: vec![],
            current_section: vec![],
        },
        &mut TrimSpaces,
    ];
    for rule in rules.into_iter() {
        rule.visit_document_mut(&mut doc);
    }
    // Special handling for fallible rules.
    let mut rule = SortKey { error: None };
    rule.visit_document_mut(&mut doc);
    if let Some(e) = rule.error {
        return Err(e);
    }

    Ok(doc.to_string())
}

impl From<TomlError> for ErrorKind {
    fn from(_: TomlError) -> Self {
        ErrorKind::ParseError
    }
}

/// Sort key names alphabetically within each section, with the exception of the
/// `[package]` section.
///
/// In `[package]` section,
/// put the `name` and `version` keys in that order at the top of that section,
/// followed by the remaining keys other than `description` in alphabetical order,
/// followed by the `description` at the end of that section.
struct SortKey {
    error: Option<ErrorKind>,
}

/// Put the `[package]` section at the top of the file
struct SortSection {
    /// `cargo-edit` uses a `position` field to put tables back in their original order when
    /// serialising. We should reset this field after sorting.
    current_position: usize,
}

/// Put a blank line between the last key-value pair in a section and the header of the next
/// section.
///
/// Do not place a blank line between section headers and the key-value pairs in that section, or
/// between key-value pairs in a section.
///
/// Should be applied after `SortSection`.
struct BlankLine {
    trimming: bool,
}

/// Trim unnecessary spaces.
///
/// Note: this is not included in the Style Guide.
struct TrimSpaces;

/// Don't use quotes around any standard key names; use bare keys. Only use quoted
/// keys for non-standard keys whose names require them, and avoid introducing such
/// key names when possible.
///
/// Put a single space both before and after the = between a key and value.
/// Do not indent any key names; start all key names at the start of a line.
struct KeyValue;

/// Use multi-line strings (rather than newline escape sequences) for any string values
/// that include multiple lines, such as the crate description.
struct MultiLine;

/// For array values, such as a list of authors, put the entire list on the same line as the key,
/// if it fits. Otherwise, use block indentation: put a newline after the opening square bracket,
/// indent each item by one indentation level, put a comma after each item (including the last),
/// and put the closing square bracket at the start of a line by itself after the last item.
///
/// ```toml
/// authors = [
///     "A Uthor <a.uthor@example.org>",
///     "Another Author <author@example.net>",
/// ]
///```
struct WrapArray {
    max_width: usize,
}

/// For table values, such as a crate dependency with a path, write the entire
/// table using curly braces and commas on the same line as the key if it fits. If
/// the entire table does not fit on the same line as the key, separate it out into
/// a separate section with key-value pairs:
///
/// ```toml
/// [dependencies]
/// crate1 = { path = "crate1", version = "1.2.3" }
///
/// [dependencies.extremely_long_crate_name_goes_here]
/// path = "extremely_long_path_name_goes_right_here"
/// version = "4.5.6"
/// ```
struct FormatInlineTable {
    max_width: usize,
    /// Must be `InlineTable`
    long_tables: Vec<(Vec<String>, String, Item)>,
    current_section: Vec<String>,
}

impl VisitMut for SortKey {
    fn visit_document_mut(&mut self, doc: &mut Document) {
        doc.as_table_mut().iter_mut().for_each(|(key, section)| {
            if key == "package" {
                let table = match section.as_table_mut() {
                    Some(table) => table,
                    None => {
                        // package should be a table
                        self.error = Some(ErrorKind::ParseError);
                        return;
                    }
                };
                // "name" is the first, "version" is the second, "description" is the last
                // everything else is sorted alphabetically
                table.sort_values_by(|k1, _, k2, _| match (k1.get(), k2.get()) {
                    ("name", _) => Ordering::Less,
                    (_, "name") => Ordering::Greater,
                    ("version", _) => Ordering::Less,
                    (_, "version") => Ordering::Greater,
                    ("description", _) => Ordering::Greater,
                    (_, "description") => Ordering::Less,
                    _ => k1.cmp(k2),
                })
            } else {
                self.visit_item_mut(section)
            }
        });
    }

    fn visit_table_like_mut(&mut self, table: &mut dyn TableLike) {
        table.sort_values();
    }
}

impl BlankLine {
    /// trim blank lines at the beginning and end
    fn trim_blank_lines(s: &str) -> String {
        if !s.contains('\n') {
            return s.to_string();
        }

        let num_lines = s.lines().count();

        if let Some((first_line, _)) = s.lines().find_position(|line| !line.trim().is_empty()) {
            // last_line may be equal to first_line
            let (mut last_line, _) = s
                .lines()
                .rev()
                .find_position(|line| !line.trim().is_empty())
                .unwrap();
            last_line = num_lines - last_line;
            s.lines()
                .skip(first_line)
                .take(last_line - first_line)
                .join("\n")
                + "\n"
        } else {
            String::new()
        }
    }

    fn trim_decor_blank_lines(decor: &mut Decor) {
        if let Some(prefix) = decor.prefix().map(raw_string_as_str) {
            decor.set_prefix(Self::trim_blank_lines(prefix));
        }
        if let Some(suffix) = decor.suffix().map(raw_string_as_str) {
            decor.set_suffix(Self::trim_blank_lines(suffix));
        }
    }
}

impl VisitMut for BlankLine {
    fn visit_document_mut(&mut self, doc: &mut Document) {
        doc.as_table_mut()
            .iter_mut()
            .for_each(|(mut key, section)| {
                Self::trim_decor_blank_lines(key.decor_mut());
                self.visit_item_mut(section);
            });

        self.trimming = false;
        doc.as_table_mut()
            .iter_mut()
            .skip(1)
            .for_each(|(_, section)| self.visit_item_mut(section))
    }

    fn visit_table_mut(&mut self, table: &mut Table) {
        if self.trimming {
            Self::trim_decor_blank_lines(table.decor_mut());
            table.iter_mut().for_each(|(mut key, _)| {
                Self::trim_decor_blank_lines(key.decor_mut());
            });
        } else {
            let decor = table.decor_mut();
            decor.set_prefix(format!(
                "\n{}",
                decor.prefix().map(raw_string_as_str).unwrap_or_default()
            ));
        }
    }
}

impl KeyValue {
    /// Bare keys can contain ASCII letters, ASCII digits, underscores, and dashes `(A-Za-z0-9_-)`.
    fn can_be_bare_key(key: &str) -> bool {
        key.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }
}

impl VisitMut for KeyValue {
    fn visit_table_like_kv_mut(&mut self, mut key: KeyMut<'_>, value: &mut Item) {
        let original_prefix = key
            .decor()
            .prefix()
            .map(raw_string_as_str)
            .map(String::from);
        if Self::can_be_bare_key(key.get()) {
            // will remove decors and set the key to the bare key
            key.fmt();
        } else {
            // add a space after the key
            key.decor_mut().set_suffix(" ");
        }
        // start all key names at the start of a line, but preserve comments
        if let Some(prefix) = original_prefix {
            key.decor_mut()
                .set_prefix(prefix.trim_end_matches(|c: char| c.is_whitespace() && c != '\n'));
        }

        if let Some(v) = value.as_value_mut() {
            v.decor_mut().set_prefix(" ");
        }

        self.visit_item_mut(value);
    }
}

impl VisitMut for SortSection {
    fn visit_document_mut(&mut self, doc: &mut Document) {
        // put package at the beginning, others unchanged
        doc.as_table_mut().sort_values_by(|k1, _, k2, _| {
            if k1.get() == "package" {
                Ordering::Less
            } else if k2.get() == "package" {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        doc.as_table_mut().iter_mut().for_each(|(_, section)| {
            self.visit_item_mut(section);
        });
    }

    fn visit_table_mut(&mut self, table: &mut Table) {
        table.set_position(self.current_position);
        self.current_position += 1;
        for (_, v) in table.iter_mut().sorted_by_key(|(k, _)| k.to_string()) {
            self.visit_item_mut(v);
        }
    }
}

impl VisitMut for MultiLine {
    fn visit_string_mut(&mut self, s: &mut Formatted<String>) {
        s.fmt();
    }
}

impl VisitMut for WrapArray {
    fn visit_table_like_kv_mut(&mut self, key: KeyMut<'_>, node: &mut Item) {
        if let Some(array) = node.as_array_mut() {
            // Format to [item1, item2, ...]
            array.fmt();
            // Length of key doesn't include decor. Length of array does. So we add 2 (" =").
            if key.get().len() + 2 + array.to_string().len() > self.max_width {
                array.iter_mut().for_each(|item| {
                    item.decor_mut().set_prefix("\n    ");
                });
                array
                    .iter_mut()
                    .last()
                    .unwrap()
                    .decor_mut()
                    .set_suffix("\n");
            }
        }
        self.visit_item_mut(node);
    }
}

impl VisitMut for FormatInlineTable {
    fn visit_document_mut(&mut self, doc: &mut Document) {
        doc.as_table_mut().iter_mut().for_each(|(key, section)| {
            self.current_section = vec![key.to_owned()];
            self.visit_table_like_kv_mut(key, section);
        });

        let mut long_tables = vec![];
        std::mem::swap(&mut self.long_tables, &mut long_tables);

        long_tables
            .into_iter()
            .for_each(|(sections, key, table)| match table {
                Item::Value(Value::InlineTable(table)) => {
                    let mut section = doc.as_item_mut();
                    for key in sections {
                        section = &mut section[&key]
                    }
                    section[&key] = Item::Table(table.into_table());
                }
                _ => unreachable!(),
            });
    }

    fn visit_table_like_mut(&mut self, table: &mut dyn TableLike) {
        let mut long_table_keys = vec![];

        table.iter_mut().for_each(|(key, node)| {
            if let Some(table) = node.as_inline_table_mut() {
                // Format to { k1 = v1, k2 = v2, ...}
                table.fmt();
                // Length of key doesn't include decor. Length of array does. So we add 2 (" =").
                if key.get().len() + 2 + table.to_string().len() > self.max_width {
                    long_table_keys.push(key.get().to_owned());
                }
            }
        });

        long_table_keys.into_iter().sorted().for_each(|key| {
            let item = table.remove(&key).unwrap();
            self.long_tables
                .push((self.current_section.clone(), key, item));
        });

        table.iter_mut().for_each(|(key, node)| {
            self.current_section.push(key.to_owned());
            self.visit_item_mut(node);
            self.current_section.pop();
        });
    }
}

impl TrimSpaces {
    fn trim_block(s: &str) -> String {
        let s = s.trim();
        if s.is_empty() {
            return String::new();
        }

        let s: String = s
            .lines()
            .into_iter()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(format!("{trimmed}"))
                }
            })
            .join("\n");

        format!("{}\n", s)
    }

    fn trim_suffix(s: &str) -> String {
        let s = s.trim();
        if s.is_empty() {
            String::new()
        } else {
            format!(" {}", s)
        }
    }
}

impl VisitMut for TrimSpaces {
    fn visit_document_mut(&mut self, node: &mut Document) {
        self.visit_table_mut(node);

        let set_prefix = |decor: &mut Decor, i: usize| {
            if let Some(prefix) = decor.prefix().map(raw_string_as_str) {
                let prefix = format!(
                    "{}{}",
                    if i == 0 { "" } else { "\n" },
                    Self::trim_block(prefix)
                );
                decor.set_prefix(prefix);
            }
        };
        let table = node.as_table_mut();
        for (i, (_, item)) in table.iter_mut().enumerate() {
            if let Some(table) = item.as_table_mut() {
                set_prefix(table.decor_mut(), i);
            } else if let Some(arr) = item.as_array_of_tables_mut() {
                for table in arr.iter_mut() {
                    set_prefix(table.decor_mut(), i);
                }
            }
        }

        let trailing = raw_string_as_str(node.trailing());
        if !trailing.trim().is_empty() {
            let trailing = Self::trim_block(trailing);
            node.set_trailing(&format!("\n{trailing}"));
        } else {
            node.set_trailing("");
        }
    }

    fn visit_table_mut(&mut self, node: &mut Table) {
        let decor = node.decor_mut();
        if let Some(prefix) = decor.prefix().map(raw_string_as_str) {
            decor.set_prefix(format!("\n{}", Self::trim_block(prefix)));
        }
        if let Some(suffix) = decor.suffix().map(raw_string_as_str) {
            decor.set_suffix(Self::trim_suffix(suffix));
        }
        self.visit_table_like_mut(node);
    }

    fn visit_table_like_kv_mut(&mut self, mut key: KeyMut<'_>, value: &mut Item) {
        let decor = key.decor_mut();
        if let Some(prefix) = decor.prefix().map(raw_string_as_str) {
            decor.set_prefix(format!("{}", Self::trim_block(prefix)));
        }

        if let Some(value) = value.as_value_mut() {
            let decor = value.decor_mut();
            if let Some(suffix) = decor.suffix().map(raw_string_as_str) {
                decor.set_suffix(Self::trim_suffix(suffix));
            }
        }
        self.visit_item_mut(value);
    }
}

/// Note: in `Document::from_str`, the document is despanned, so we can safely unwrap `as_str`
/// when handling `RawString`.
fn raw_string_as_str(raw_string: &RawString) -> &str {
    raw_string.as_str().expect("should already be despanded")
}
