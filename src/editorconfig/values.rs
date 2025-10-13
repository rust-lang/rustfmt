//! Enums contain all possible value types even if they are never constructed, to allow easier
//! modifications of value mappings
use std::fmt::Display;

use crate::NewlineStyle;

#[derive(Clone, Debug)]
pub(super) enum EOLControllChar {
    Lf,
    #[allow(dead_code)]
    Cr,
    Crlf,
}

impl EOLControllChar {
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

impl Display for EOLControllChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EOLControllChar::Lf => "lf",
            EOLControllChar::Cr => "cr",
            EOLControllChar::Crlf => "crlf",
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

#[cfg(test)]
mod unit_tests {
    use crate::editorconfig::{CharSet, EOLControllChar, IndentSize, IndentStyle};

    #[test]
    fn eol_controll_char() {
        assert_eq!(EOLControllChar::Crlf.to_string(), "crlf");
        assert_eq!(EOLControllChar::Cr.to_string(), "cr");
        assert_eq!(EOLControllChar::Lf.to_string(), "lf");
    }

    #[test]
    fn char_set() {
        assert_eq!(CharSet::Latin1.to_string(), "latin1");
        assert_eq!(CharSet::UTF8.to_string(), "utf-8");
        assert_eq!(CharSet::UTF16_BE.to_string(), "utf-16be");
        assert_eq!(CharSet::UTF16_LE.to_string(), "utf-16le");
        assert_eq!(CharSet::UTF8_BOM.to_string(), "utf-8-bom");
    }

    #[test]
    fn indent_style() {
        assert_eq!(IndentStyle::Tab.to_string(), "tab");
        assert_eq!(IndentStyle::Space.to_string(), "space");
    }

    #[test]
    fn indent_size() {
        assert_eq!(IndentSize::Tab.to_string(), "tab");
        assert_eq!(IndentSize::Columns(1).to_string(), "1");
        assert_eq!(IndentSize::Columns(3265444).to_string(), "3265444");
        assert_eq!(
            IndentSize::Columns(u32::MAX).to_string(),
            u32::MAX.to_string()
        );
    }
}
