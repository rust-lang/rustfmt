use ignore::gitignore::{Gitignore, GitignoreBuilder};

use crate::config::{FileName, IgnoreList};

pub(crate) struct IgnorePathSet {
    ignore_set: Gitignore,
}

impl IgnorePathSet {
    pub(crate) fn from_ignore_list(ignore_list: &IgnoreList) -> Result<Self, ignore::Error> {
        let mut ignore_builder = GitignoreBuilder::new(ignore_list.rustfmt_toml_path());

        for ignore_path in ignore_list {
            ignore_builder.add_line(None, &ignore_path.to_string_lossy())?;
        }

        Ok(Self {
            ignore_set: ignore_builder.build()?,
        })
    }

    pub(crate) fn is_match(&self, file_name: &FileName) -> bool {
        match file_name {
            FileName::Stdin => false,
            FileName::Real(p) => self
                .ignore_set
                .matched_path_or_any_parents(p, false)
                .is_ignore(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::IgnorePathSet;
    use crate::config::{Config, FileName};

    #[test]
    fn test_ignore_path_set() {
        match option_env!("CFG_RELEASE_CHANNEL") {
            // this test requires nightly
            None | Some("nightly") => {
                let config =
                    Config::from_toml(r#"ignore = ["foo.rs", "bar_dir/*"]"#, Path::new(""))
                        .unwrap();
                let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();

                assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/foo.rs"))));
                assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz.rs"))));
                assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/bar.rs"))));
            }
            _ => (),
        };
    }
}
