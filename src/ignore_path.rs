use ignore::gitignore;

use crate::config::{FileName, IgnoreList};

pub(crate) struct IgnorePathSet {
    ignore_set: gitignore::Gitignore,
}

impl IgnorePathSet {
    pub(crate) fn from_ignore_list(ignore_list: &IgnoreList) -> Result<Self, ignore::Error> {
        let mut ignore_builder = gitignore::GitignoreBuilder::new(ignore_list.rustfmt_toml_path());

        for ignore_path in ignore_list {
            ignore_builder.add_line(None, ignore_path.to_str().unwrap())?;
        }

        Ok(IgnorePathSet {
            ignore_set: ignore_builder.build()?,
        })
    }

    pub(crate) fn is_match(&self, file_name: &FileName) -> bool {
        match file_name {
            FileName::Stdin => false,
            FileName::Real(p) => {
                if p.is_absolute() && !p.starts_with(self.ignore_set.path()) {
                    false
                } else {
                    self.ignore_set
                        .matched_path_or_any_parents(p, false)
                        .is_ignore()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rustfmt_config_proc_macro::nightly_only_test;

    #[nightly_only_test]
    #[test]
    fn test_ignore_path_set() {
        use crate::config::{Config, FileName};
        use crate::ignore_path::IgnorePathSet;
        use std::path::{Path, PathBuf};

        let config = Config::from_toml(
            r#"ignore = ["foo.rs", "bar_dir/*"]"#,
            Path::new("./rustfmt.toml"),
        )
        .unwrap();
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();

        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/foo.rs"))));
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz.rs"))));
        assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/bar.rs"))));
    }

    #[nightly_only_test]
    #[test]
    fn test_negated_ignore_path_set() {
        use crate::config::{Config, FileName};
        use crate::ignore_path::IgnorePathSet;
        use std::path::{Path, PathBuf};

        let config = Config::from_toml(
            r#"ignore = ["foo.rs", "bar_dir/*", "!bar_dir/*/what.rs"]"#,
            Path::new("./rustfmt.toml"),
        )
        .unwrap();
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/what.rs"))));
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz/a.rs"))));
        assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz/what.rs"))));
    }

    #[nightly_only_test]
    #[test]
    fn test_ignore_path_outside_root() {
        //See: https://github.com/rust-lang/rustfmt/issues/6843
        use crate::config::{Config, FileName};
        use crate::ignore_path::IgnorePathSet;

        let config_path = std::env::temp_dir().join("a").join("rustfmt.toml");
        let file_path = std::env::temp_dir().join("b").join("foo.rs");

        let config = Config::from_toml(r#"ignore = ["bar.rs"]"#, &config_path).unwrap();
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();

        //should not panic
        assert!(!ignore_path_set.is_match(&FileName::Real(file_path)));
    }
}
