use globset::{self, Glob, GlobSet, GlobSetBuilder};

use crate::config::{FileName, IgnoreList};

pub struct IgnorePathSet {
    ignore_glob_set: GlobSet,
}

impl IgnorePathSet {
    pub fn from_ignore_list(ignore_list: &IgnoreList) -> Result<Self, globset::Error> {
        let mut globset_builder = GlobSetBuilder::new();
        for ignore_path in ignore_list {
            globset_builder.add(Glob::new(&ignore_path.to_string_lossy())?);
        }
        Ok(IgnorePathSet {
            ignore_glob_set: globset_builder.build()?,
        })
    }

    pub fn is_match(&self, file_name: &FileName) -> bool {
        match file_name {
            FileName::Stdin => false,
            FileName::Real(p) => self.ignore_glob_set.is_match(p),
        }
    }
}
