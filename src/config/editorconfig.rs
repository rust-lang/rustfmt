//! This module adapts between .editorconfig values and rustfmt’s formatting options.

use std::path::Path;

use ec4rs::{
    Properties, properties_of,
    property::{
        Charset, EndOfLine, FinalNewline, IndentSize, IndentStyle, MaxLineLen, TabWidth,
        TrimTrailingWs,
    },
};

use crate::{NewlineStyle, config::PartialConfig};

/// Reads any applicable .editorconfig values for the given file,
/// and converts it into rustfmt’s config options.
/// Returns None if there is no applicable .editorconfig.
pub fn editorconfig_configuration_for(
    file: impl AsRef<Path>,
) -> Result<Option<PartialConfig>, String> {
    let mut properties = properties_of(file).map_err(|e| e.to_string())?;
    properties.use_fallbacks();
    Ok(Some(properties.try_into()?))
}

impl TryFrom<Properties> for PartialConfig {
    type Error = String;

    fn try_from(properties: Properties) -> Result<Self, Self::Error> {
        let mut config = PartialConfig::default();

        // 1. Ensure that "unconfigurable" keys have the values we expect, if present.
        // Only utf-8 charsets are permitted.
        let charset = get_editorconfig_property::<Charset>(&properties)?;
        if let Some(charset) = charset {
            if ![Charset::Utf8, Charset::Utf8Bom].contains(&charset) {
                return Err(format!("Invalid charset '{charset}'",));
            }
        }
        // Always true.
        let trim_trailing_whitespace = get_editorconfig_property::<TrimTrailingWs>(&properties)?;
        if let Some(TrimTrailingWs::Value(false)) = trim_trailing_whitespace {
            return Err(format!("trim_trailing_whitespace must not be false",));
        }
        // Always true.
        let insert_final_newline = get_editorconfig_property::<FinalNewline>(&properties)?;
        if let Some(FinalNewline::Value(false)) = insert_final_newline {
            return Err(format!("insert_final_newline must not be false",));
        }

        // 2. Actually parse configurable keys.
        let indent_style = get_editorconfig_property::<IndentStyle>(&properties)?;
        let indent_size = get_editorconfig_property::<IndentSize>(&properties)?;
        let tab_width = get_editorconfig_property::<TabWidth>(&properties)?;
        let end_of_line = get_editorconfig_property::<EndOfLine>(&properties)?;
        let max_line_len = get_editorconfig_property::<MaxLineLen>(&properties)?;

        if let Some(indent_style) = indent_style {
            config.hard_tabs = Some(indent_style == IndentStyle::Tabs);
        }
        if let Some(IndentSize::Value(indent_size)) = indent_size {
            config.tab_spaces = Some(indent_size);
        }
        // Note: If the tab width is not set, the specification says to use
        //       "the tab size set by the editor."
        //       We interpret this to mean "the default/fallback value",
        //       and don’t specify tab_spaces in this case.
        if let Some(IndentSize::UseTabWidth) = indent_size {
            if let Some(TabWidth::Value(indent_size)) = tab_width {
                config.tab_spaces = Some(indent_size);
            }
        }
        if let Some(end_of_line) = end_of_line {
            config.newline_style = match end_of_line {
                EndOfLine::Lf => Some(NewlineStyle::Unix),
                EndOfLine::CrLf => Some(NewlineStyle::Windows),
                EndOfLine::Cr => return Err("'end_of_line=cr' is not supported".to_owned()),
            };
        }
        if let Some(MaxLineLen::Value(max_line_len)) = max_line_len {
            config.max_width = Some(max_line_len);
        }

        Ok(config)
    }
}

/// Improved wrapper around [`ec4rs::Properties::get`].
fn get_editorconfig_property<T: ec4rs::PropertyKey + ec4rs::PropertyValue>(
    properties: &ec4rs::Properties,
) -> Result<Option<T>, String> {
    properties.get::<T>().map_or_else(
        |raw| {
            if raw.filter_unset().is_unset() {
                Ok(None)
            } else {
                Err(format!("Invalid value {raw}"))
            }
        },
        |value| Ok(Some(value)),
    )
}
