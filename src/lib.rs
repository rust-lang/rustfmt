#![deny(rust_2018_idioms)]
#![warn(unreachable_pub)]
#![feature(cell_leak)]

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;

pub use crate::config::{
    load_config, CliOptions, Config, Edition, FileLines, FileName, NewlineStyle, Range,
};
pub use crate::emitter::rustfmt_diff::{ModifiedChunk, ModifiedLines};
pub use crate::format_report_formatter::{FormatReportFormatter, FormatReportFormatterBuilder};
pub use crate::formatting::report::{FormatReport, FormatResult};

use crate::formatting::format_input_inner;
use crate::{emitter::Verbosity, result::OperationError};

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "emitter")]
pub mod emitter;

mod format_report_formatter;
mod formatting;
mod release_channel;
pub mod result;

#[cfg(test)]
mod test;

/// Configures how rustfmt operates during formatting.
#[derive(Clone, Copy, Default)]
pub struct OperationSetting {
    /// If set to `true`, format sub-modules which are defined in the given input.
    pub recursive: bool,
    pub verbosity: Verbosity,
}

/// The main entry point for Rustfmt. Formats the given input according to the
/// given config. `out` is only necessary if required by the configuration.
pub fn format(
    input: Input,
    config: &Config,
    operation_setting: OperationSetting,
) -> Result<FormatReport, OperationError> {
    format_input_inner(input, config, operation_setting)
}

pub fn format_inputs<'a>(
    inputs: impl Iterator<Item = (Input, &'a Config)>,
    operation_setting: OperationSetting,
) -> Result<FormatReport, OperationError> {
    let mut format_report = FormatReport::new();
    for (input, config) in inputs {
        let report = format(input, config, operation_setting)?;
        format_report.merge(report);
    }
    Ok(format_report)
}

/// The input to rustfmt.
#[derive(Debug)]
pub enum Input {
    /// A file on the filesystem.
    File(PathBuf),
    /// A UTF-8 string, in many cases from stdin.
    Text(String),
}
