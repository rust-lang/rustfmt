use itertools::Itertools;
use std::cmp::Ordering;
use toml_edit::{
    visit_mut::*, Decor, Document, Formatted, Item, KeyMut, Table, TableLike, TomlError, Value,
};

use crate::{Config, ErrorKind};

pub(crate) fn format_cargo_toml_inner(content: &str, config: &Config) -> Result<String, ErrorKind> {
    let mut doc = content.parse::<toml_edit::Document>()?;
    let rules: Vec<Box<dyn VisitMut>> = vec![
        Box::new(SortSection {
            current_position: 0,
        }),
        Box::new(SortKey),
        Box::new(BlankLine { trimming: true }),
        Box::new(KeyValue),
        Box::new(MultiLine),
        Box::new(WrapArray {
            max_width: config.max_width(),
        }),
        Box::new(FormatInlineTable {
            max_width: config.max_width(),
            long_tables: vec![],
            current_section: String::new(),
        }),
    ];
    for mut rule in rules.into_iter() {
        rule.visit_document_mut(&mut doc);
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
struct SortKey;

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
    long_tables: Vec<(String, String, Item)>,
    current_section: String,
}

impl VisitMut for SortKey {
    fn visit_document_mut(&mut self, doc: &mut Document) {
        doc.as_table_mut().iter_mut().for_each(|(key, section)| {
            if key == "package" {
                let table = section.as_table_mut().expect("package should be a table");
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
        let prefix = decor.prefix().unwrap_or("").to_owned();
        let suffix = decor.suffix().unwrap_or("").to_owned();
        decor.set_prefix(Self::trim_blank_lines(prefix.as_str()));
        decor.set_suffix(Self::trim_blank_lines(suffix.as_str()));
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
            let prefix = decor.prefix().unwrap_or("").to_owned();
            decor.set_prefix("\n".to_owned() + &prefix);
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
        let prefix = key.decor().prefix().unwrap_or("").to_owned();
        if Self::can_be_bare_key(key.get()) {
            // will remove decors and set the key to the bare key
            key.fmt();
        } else {
            // add a space after the key
            key.decor_mut().set_suffix(" ");
        }
        // start all key names at the start of a line, but preserve comments
        key.decor_mut()
            .set_prefix(prefix.trim_end_matches(|c: char| c.is_whitespace() && c != '\n'));

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
            self.current_section = key.to_owned();
            self.visit_table_like_kv_mut(key, section);
        });

        let mut long_tables = vec![];
        std::mem::swap(&mut self.long_tables, &mut long_tables);

        println!("\n\n long tables\n {:?}\n\n", long_tables);
        long_tables.into_iter().for_each(|(section, key, table)| {
            match table {
                Item::Value(Value::InlineTable(table)) => {
                    // let table = format!("[{}]\n{}",key,table).parse::<Table>().unwrap();
                    doc[&section][&key] = Item::Table(table.into_table());
                }
                _ => unreachable!(),
            }
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

        long_table_keys.into_iter().for_each(|key| {
            let item = table.remove(&key).unwrap();
            println!("removed long item {:?}", item);
            self.long_tables
                .push((self.current_section.clone(), key, item));
        });

        table.iter_mut().for_each(|(_, node)| {
            self.visit_item_mut(node);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_toml() {
        #[rustfmt::skip]
        let s = r#"


[[bin]]
        "aa" = 1
        'bb' = 2
        "啊"=1
[package]
    version = 1
    description = "a\nb\nhaha"
    name = 3

# comment 1

    arr1 = [1,
        2,3]

# comment 2

    arr2 = ["11111111111111111111111111111111111111111111111111111111111111111111111111111111","1111111111111111111111111111111111111111111111111111111111111111111111111111111"]

[dependencies]
        extremely_long_crate_name_goes_here = {path = "extremely_long_path_name_goes_right_here",version = "4.5.6"}
        crate1 = {            path = "crate1",version = "1.2.3"                   }

[[bin]]
        d = "git-rustfmt"
        c = "src/git-rustfmt/main.rs""#;

        let formatted = format_cargo_toml_inner(s, &Default::default()).unwrap();

        #[rustfmt::skip]
        let expected = r#"[package]
name = 3
version = 1
# comment 1
arr1 = [1, 2, 3]
# comment 2
arr2 = [
    "11111111111111111111111111111111111111111111111111111111111111111111111111111111",
    "1111111111111111111111111111111111111111111111111111111111111111111111111111111"
]
description = """
a
b
haha"""

[[bin]]
aa = 1
bb = 2
"啊" = 1

[[bin]]
c = "src/git-rustfmt/main.rs"
d = "git-rustfmt"

[dependencies]
crate1 = { path = "crate1", version = "1.2.3" }

[dependencies.extremely_long_crate_name_goes_here]
path = "extremely_long_path_name_goes_right_here"
version = "4.5.6"
"#;

        println!("{}", formatted);
        assert_eq!(formatted, expected);
    }
}
