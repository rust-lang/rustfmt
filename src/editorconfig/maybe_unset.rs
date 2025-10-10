use std::fmt::Display;

/// The EditorConfig equivalent of Option<T>
#[derive(Clone)]
pub(super) enum MaybeUnset<T> {
    Unset,
    Set(T),
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

impl<T: Display> Display for MaybeUnset<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeUnset::Unset => f.write_str("unset"),
            MaybeUnset::Set(val) => val.fmt(f),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UnsetBehaviour {
    Omit,
    Emit,
}

impl Default for UnsetBehaviour {
    fn default() -> Self {
        Self::Omit
    }
}
