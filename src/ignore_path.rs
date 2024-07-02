use crate::config::{FileName, IgnoreList};
use ignore::gitignore;
use std::path::{Path, PathBuf};

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
            FileName::Real(p) => self
                .ignore_set
                .matched_path_or_any_parents(p, false)
                .is_ignore(),
        }
    }
}

/// Determine if input from stdin should be ignored by rustfmt.
/// See the `ignore` configuration options for details on specifying ignore files.
pub fn is_std_ignored(file_hint: Option<PathBuf>, ignore_list: &IgnoreList) -> bool {
    // trivially return false, because no files are ignored
    if ignore_list.is_empty() {
        return false;
    }

    // trivially return true, because everything is ignored when "/" is in the ignore list
    if ignore_list.contains(Path::new("/")) {
        return true;
    }

    // See if the hinted stdin input is an ignored file.
    if let Some(std_file_hint) = file_hint {
        let file = FileName::Real(std_file_hint);
        match IgnorePathSet::from_ignore_list(ignore_list) {
            Ok(ignore_set) if ignore_set.is_match(&file) => {
                debug!("{:?} is ignored", file);
                return true;
            }
            _ => {}
        }
    }
    false
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

        let config =
            Config::from_toml(r#"ignore = ["foo.rs", "bar_dir/*"]"#, Path::new("")).unwrap();
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
            Path::new(""),
        )
        .unwrap();
        let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/what.rs"))));
        assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz/a.rs"))));
        assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz/what.rs"))));
    }

    #[test]
    fn test_is_std_ignored() {
        use serde_json;
        use std::path::PathBuf;

        use super::is_std_ignored;
        use crate::config::IgnoreList;

        let ignore_list: IgnoreList = serde_json::from_str(r#"["foo.rs","bar_dir/*"]"#).unwrap();
        assert!(is_std_ignored(Some(PathBuf::from("foo.rs")), &ignore_list));
        assert!(is_std_ignored(
            Some(PathBuf::from("src/foo.rs")),
            &ignore_list
        ));
        assert!(is_std_ignored(
            Some(PathBuf::from("bar_dir/bar/bar.rs")),
            &ignore_list
        ));

        assert!(!is_std_ignored(Some(PathBuf::from("baz.rs")), &ignore_list));
        assert!(!is_std_ignored(
            Some(PathBuf::from("src/baz.rs")),
            &ignore_list
        ));
        assert!(!is_std_ignored(
            Some(PathBuf::from("baz_dir/baz/baz.rs")),
            &ignore_list
        ));
        assert!(!is_std_ignored(None, &ignore_list));
    }
}
