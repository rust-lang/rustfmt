//! This module contains types and functions to support formatting specific macros.

use itertools::Itertools;
use std::{fmt, str};

use serde::{Deserialize, Serialize};
use serde_json as json;
use thiserror::Error;

/// Defines the name of a macro.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
pub struct MacroName(String);

impl fmt::Display for MacroName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<MacroName> for String {
    fn from(other: MacroName) -> Self {
        other.0
    }
}

/// A set of macro names.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MacroNames(Vec<MacroName>);

impl fmt::Display for MacroNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().format(", "))
    }
}

impl MacroNames {
    /// Return the underlying macro names, as an iterator of strings.
    pub(crate) fn into_name_strings(self) -> impl Iterator<Item = String> {
        self.0.into_iter().map(Into::into)
    }
}

#[derive(Error, Debug)]
pub enum MacroNamesError {
    #[error("{0}")]
    Json(json::Error),
}

// This impl is needed for `Config::override_value` to work for use in tests.
impl str::FromStr for MacroNames {
    type Err = MacroNamesError;

    fn from_str(s: &str) -> Result<MacroNames, Self::Err> {
        json::from_str(s).map_err(MacroNamesError::Json)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn macro_names_from_str() {
        let macro_names = MacroNames::from_str(r#"["foo", "bar"]"#).unwrap();
        assert_eq!(
            macro_names,
            MacroNames(
                [MacroName("foo".to_owned()), MacroName("bar".to_owned())]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn macro_names_display() {
        let macro_names = MacroNames::from_str(r#"["foo", "bar"]"#).unwrap();
        assert_eq!(format!("{}", macro_names), "foo, bar");
    }
}
