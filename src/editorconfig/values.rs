//! Enums contain all possible value types even if they are never constructed, to allow easier
//! modifications of value mappings
use std::fmt::Display;

use crate::NewlineStyle;

#[derive(Clone, Debug)]
pub(super) enum EOFControllChar {
    Lf,
    #[allow(dead_code)]
    Cr,
    Crlf,
}

impl EOFControllChar {
    pub(super) fn from_newline_style(value: &NewlineStyle) -> Option<Self> {
        match value {
            NewlineStyle::Auto => None,
            NewlineStyle::Windows => Some(Self::Crlf),
            NewlineStyle::Unix => Some(Self::Lf),
            NewlineStyle::Native => Some(match cfg!(windows) {
                true => Self::Crlf,
                false => Self::Lf,
            }),
        }
    }
}

impl Display for EOFControllChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EOFControllChar::Lf => "lf",
            EOFControllChar::Cr => "cr",
            EOFControllChar::Crlf => "crlf",
        })
    }
}

#[derive(Clone)]
pub(super) enum CharSet {
    #[allow(dead_code)]
    Latin1,
    UTF8,
    /// Discouraged
    #[allow(non_camel_case_types, dead_code)]
    UTF8_BOM,
    #[allow(non_camel_case_types, dead_code)]
    /// UTF-16 Big Endian
    UTF16_BE,
    #[allow(non_camel_case_types, dead_code)]
    /// UTF-16 Little Endian
    UTF16_LE,
}

impl CharSet {
    pub(super) fn is_utf8(&self) -> bool {
        match self {
            Self::UTF8 => true,
            _ => false,
        }
    }
}

impl Display for CharSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CharSet::Latin1 => "latin1",
            CharSet::UTF8 => "utf-8",
            CharSet::UTF8_BOM => "utf-8-bom",
            CharSet::UTF16_BE => "utf-16be",
            CharSet::UTF16_LE => "utf-16le",
        })
    }
}

#[derive(Clone)]
pub(super) enum IndentStyle {
    Tab,
    Space,
}

impl IndentStyle {
    pub(super) fn is_tab(&self) -> bool {
        match self {
            IndentStyle::Tab => true,
            _ => false,
        }
    }
    pub(super) fn is_space(&self) -> bool {
        match self {
            IndentStyle::Space => true,
            _ => false,
        }
    }
}

impl Display for IndentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            IndentStyle::Tab => "tab",
            IndentStyle::Space => "space",
        })
    }
}

#[derive(Clone)]
pub(super) enum IndentSize {
    Tab,
    #[allow(dead_code)]
    Columns(u32),
}

impl IndentSize {
    pub(super) fn is_tab(&self) -> bool {
        match self {
            IndentSize::Tab => true,
            _ => false,
        }
    }
}

impl Display for IndentSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndentSize::Tab => f.write_str("tab"),
            IndentSize::Columns(val) => val.fmt(f),
        }
    }
}
