use std::fmt::Display;

use crate::NewlineStyle;

use super::{CharSet, EOFControllChar, IndentSize, IndentStyle};

/// The EditorConfig equivalent of Option<T>
#[derive(Clone, PartialEq, Debug)]
pub(super) enum MaybeUnset<T> {
    Unset,
    Set(T),
}

impl MaybeUnset<CharSet> {
    #[allow(dead_code)]
    pub(super) fn is_utf8(&self) -> bool {
        match self {
            MaybeUnset::Set(val) => val.is_utf8(),
            _ => false,
        }
    }
}

impl MaybeUnset<IndentSize> {
    #[allow(dead_code)]
    pub(super) fn is_tab(&self) -> bool {
        match self {
            MaybeUnset::Set(val) => val.is_tab(),
            _ => false,
        }
    }
}

impl MaybeUnset<IndentStyle> {
    #[allow(dead_code)]
    pub(super) fn is_tab(&self) -> bool {
        match self {
            Self::Set(val) => val.is_tab(),
            _ => false,
        }
    }
    #[allow(dead_code)]
    pub(super) fn is_space(&self) -> bool {
        match self {
            Self::Set(val) => val.is_space(),
            _ => false,
        }
    }
}

impl PartialEq<NewlineStyle> for MaybeUnset<EOFControllChar> {
    fn eq(&self, other: &NewlineStyle) -> bool {
        match self {
            MaybeUnset::Unset => match other {
                NewlineStyle::Auto => true,
                _ => false,
            },
            MaybeUnset::Set(val) => match val {
                EOFControllChar::Lf => match other {
                    NewlineStyle::Unix => true,
                    NewlineStyle::Native => !cfg!(windows),
                    _ => false,
                },
                EOFControllChar::Cr => false,
                EOFControllChar::Crlf => match other {
                    NewlineStyle::Windows => true,
                    NewlineStyle::Native => cfg!(windows),
                    _ => false,
                },
            },
        }
    }
}
impl PartialEq<MaybeUnset<EOFControllChar>> for NewlineStyle {
    fn eq(&self, other: &MaybeUnset<EOFControllChar>) -> bool {
        other == self
    }
}

impl<T> MaybeUnset<T> {
    #[allow(dead_code)]
    pub(super) fn unwrap(self) -> T {
        match self {
            MaybeUnset::Unset => panic!("Unwrap called on an unset value."),
            MaybeUnset::Set(val) => val,
        }
    }
    #[allow(dead_code)]
    pub(super) fn is_unset(&self) -> bool {
        match self {
            Self::Unset => true,
            _ => false,
        }
    }
    #[allow(dead_code)]
    pub(super) fn is_set(&self) -> bool {
        !self.is_unset()
    }
}

impl<T> MaybeUnset<T> {
    pub(super) const UNSET_VAL: &'static str = "unset";
}

impl<T> From<Option<T>> for MaybeUnset<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => MaybeUnset::Set(val),
            None => MaybeUnset::Unset,
        }
    }
}

impl<T> From<T> for MaybeUnset<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

impl<T: Display> Display for MaybeUnset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeUnset::Unset => f.write_str("unset"),
            MaybeUnset::Set(val) => val.fmt(f),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum UnsetBehaviour {
    Omit,
    Emit,
}

impl Default for UnsetBehaviour {
    fn default() -> Self {
        Self::Omit
    }
}
