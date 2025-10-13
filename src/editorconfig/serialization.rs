use std::{fmt::Display, io::Write as _, path::PathBuf};

use crate::Config;

use super::{EditorConfig, maybe_unset::UnsetBehaviour};

#[derive(Clone, Debug)]
pub enum EditorConfigSerializationTarget {
    Stdout,
    Directory(PathBuf),
    File(PathBuf),
}

impl From<Option<PathBuf>> for EditorConfigSerializationTarget {
    fn from(value: Option<PathBuf>) -> Self {
        match value {
            Some(path) => match path.is_dir() {
                true => Self::Directory(path),
                false => Self::File(path),
            },
            None => Self::Stdout,
        }
    }
}

fn write_to_dir<S: Display>(dir: PathBuf, s: &S) {
    let mut config = dir;
    config.push(".editorconfig");
    append_to_file(&config, s)
}

fn append_to_file<S: Display>(p: &PathBuf, s: &S) {
    write!(
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(p)
            .unwrap(),
        "{}",
        s,
    )
    .unwrap()
}

pub struct EditorConfigSerializer {
    config: EditorConfig,
    unset_behaviour: UnsetBehaviour,
}

impl EditorConfigSerializer {
    pub fn new(config: EditorConfig, unset_behaviour: UnsetBehaviour) -> Self {
        Self {
            config,
            unset_behaviour,
        }
    }
    pub fn unset_omit(&mut self) {
        self.unset_behaviour = UnsetBehaviour::Omit;
    }
    pub fn unset_emit(&mut self) {
        self.unset_behaviour = UnsetBehaviour::Emit;
    }

    pub fn write_to_target(self, target: &mut EditorConfigSerializationTarget) {
        let s = self.to_string();
        match target {
            EditorConfigSerializationTarget::Stdout => {
                write!(&mut std::io::stdout().lock(), "{s}").unwrap()
            }
            EditorConfigSerializationTarget::Directory(path_buf) => {
                write_to_dir(path_buf.clone(), &s)
            }
            EditorConfigSerializationTarget::File(file) => append_to_file(file, &s),
        }
    }
}

impl From<EditorConfig> for EditorConfigSerializer {
    fn from(value: EditorConfig) -> Self {
        Self::new(value, Default::default())
    }
}

impl<'a> From<&'a Config> for EditorConfigSerializer {
    fn from(value: &'a Config) -> Self {
        EditorConfig::from(value).into()
    }
}

impl From<Config> for EditorConfigSerializer {
    fn from(value: Config) -> Self {
        EditorConfig::from(value).into()
    }
}

impl Display for EditorConfigSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.config.write_to(f, self.unset_behaviour)
    }
}

#[cfg(test)]
mod unit_tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use crate::{
        UnsetBehaviour,
        editorconfig::{
            CharSet, EOLControllChar, EditorConfig, IndentSize, IndentStyle, MaybeUnset,
        },
    };

    use super::EditorConfigSerializer;

    // (serializers, expected_resulting_kv_pairs)
    fn get_test_config_serializers(
        unset_behaviour: UnsetBehaviour,
    ) -> Vec<(EditorConfigSerializer, HashMap<&'static str, &'static str>)> {
        let mut out = vec![
            (
                EditorConfigSerializer {
                    config: EditorConfig {
                        indent_style: IndentStyle::Tab.into(),
                        indent_size: IndentSize::Tab.into(),
                        tab_width: 4.into(),
                        end_of_line: EOLControllChar::Lf.into(),
                        charset: CharSet::UTF8.into(),
                        trim_trailing_whitespace: true.into(),
                        insert_final_newline: true.into(),
                        max_line_length: 100.into(),
                    },
                    unset_behaviour,
                },
                HashMap::from([
                    ("indent_style", "tab"),
                    ("indent_size", "tab"),
                    ("tab_width", "4"),
                    ("end_of_line", "lf"),
                    ("charset", "utf-8"),
                    ("trim_trailing_whitespace", "true"),
                    ("insert_final_newline", "true"),
                    ("max_line_length", "100"),
                ]),
            ),
            // Everything is left unset
            (
                EditorConfigSerializer {
                    config: EditorConfig {
                        indent_style: MaybeUnset::Unset,
                        indent_size: MaybeUnset::Unset,
                        tab_width: MaybeUnset::Unset,
                        end_of_line: MaybeUnset::Unset,
                        charset: MaybeUnset::Unset,
                        trim_trailing_whitespace: MaybeUnset::Unset,
                        insert_final_newline: MaybeUnset::Unset,
                        max_line_length: MaybeUnset::Unset,
                    },
                    unset_behaviour,
                },
                HashMap::from([]),
            ),
            // All set
            (
                EditorConfigSerializer {
                    config: EditorConfig {
                        indent_style: IndentStyle::Space.into(),
                        indent_size: IndentSize::Columns(4).into(),
                        tab_width: 1.into(),
                        end_of_line: EOLControllChar::Crlf.into(),
                        charset: CharSet::UTF16_LE.into(),
                        trim_trailing_whitespace: false.into(),
                        insert_final_newline: false.into(),
                        max_line_length: 1.into(),
                    },
                    unset_behaviour,
                },
                HashMap::from([
                    ("indent_style", "space"),
                    ("indent_size", "4"),
                    ("tab_width", "1"),
                    ("end_of_line", "crlf"),
                    ("charset", "utf-16le"),
                    ("trim_trailing_whitespace", "false"),
                    ("insert_final_newline", "false"),
                    ("max_line_length", "1"),
                ]),
            ),
        ];
        if unset_behaviour == UnsetBehaviour::Emit {
            let unset_keys = [
                vec![],
                vec![
                    "indent_style",
                    "indent_size",
                    "tab_width",
                    "end_of_line",
                    "charset",
                    "trim_trailing_whitespace",
                    "insert_final_newline",
                    "max_line_length",
                ],
                vec![],
            ];
            out.iter_mut()
                .zip_eq(unset_keys)
                .for_each(|((_serializer, expected), unset_keys)| {
                    unset_keys.into_iter().for_each(|unset_key| {
                        assert!(
                            expected.insert(unset_key, "unset").is_none(),
                            "value already set"
                        )
                    });
                });
        }
        out
    }

    #[test]
    fn all_set_format_validity() {
        let serialized_test_configs = {
            let mut serializers = get_test_config_serializers(UnsetBehaviour::Omit);
            serializers.append(&mut get_test_config_serializers(UnsetBehaviour::Emit));
            serializers
                .into_iter()
                .map(|(serializer, _expected)| serializer.to_string())
        };
        for serialized_config in serialized_test_configs {
            let is_serialization_valid = serialized_config.lines().all(|line| {
                let lines = line.trim();
                let mut chars = line.chars();
                let first = chars.next();

                // blank line
                if lines.is_empty() {
                    return true;
                }
                // Comment line
                if first.unwrap() == '#' {
                    return true;
                }
                // Key value line
                let kv_pair = lines.split_once('=');
                if let Some((key, _val)) = kv_pair {
                    return !key.is_empty();
                }

                // Section header
                if first.unwrap() == '[' && chars.last().unwrap() == ']' {
                    return true;
                }
                false
            });
            assert!(is_serialization_valid);
        }
    }

    // Checks if the generated keynames are known to editorconfig. Should be removed/modified if
    // external keys are added to the generated config.
    #[test]
    fn key_validity() {
        let serialized_test_configs = {
            let mut serializers = get_test_config_serializers(UnsetBehaviour::Omit);
            serializers.append(&mut get_test_config_serializers(UnsetBehaviour::Emit));
            serializers
                .into_iter()
                .map(|(serializer, expected)| (serializer.to_string(), expected))
        };

        let kv_pairs = serialized_test_configs.map(|(serialized_config, expected)| {
            (
                Box::from_iter(serialized_config.lines().filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                        return None;
                    }
                    line.split_once('=')
                        .map(|(k, v)| (Box::from(k.trim()), Box::from(v.trim())))
                })),
                expected,
            )
        });

        for (pairs, expected) in kv_pairs {
            let all_correct_pairs = pairs.iter().all(|(key, val): &(Box<str>, Box<str>)| {
                println!("{key} = {val}");
                **expected.get(key.as_ref()).unwrap_or_else(|| {
                    panic!(
                        "Unkown key: {key} = {val}.\n Known keys {:?}",
                        expected.keys()
                    )
                }) == **val
            });
            assert!(all_correct_pairs, "Not all pairs had the expected values");
            // Matches the expected number of entries
            assert_eq!(
                pairs.len(),
                expected.len(),
                "The number of expected pairs ({}) did not match the found number of pairs ({})",
                expected.len(),
                pairs.len()
            );
            let unique_keys = pairs.iter().unique_by(|(key, _val)| key).count();
            assert_eq!(
                pairs.len(),
                unique_keys,
                "Not all keys were unique. Expected: {}, Unique: {unique_keys}",
                pairs.len()
            );
        }
    }
}
