use std::path::PathBuf;

use super::{format_file, read_config};
use crate::{
    formatting::modules::{ModuleResolutionError, ModuleResolutionErrorKind},
    OperationError,
};

#[test]
fn parser_errors_in_submods_are_surfaced() {
    // See also https://github.com/rust-lang/rustfmt/issues/4126
    let filename = "tests/parser/issue-4126/lib.rs";
    let file = PathBuf::from(filename);
    let exp_mod_name = "invalid";
    let (config, operation, _) = read_config(&file);
    if let Err(OperationError::ModuleResolutionError { 0: inner }) =
        format_file(&file, operation, config)
    {
        let ModuleResolutionError { module, kind } = inner;
        assert_eq!(&module, exp_mod_name);
        if let ModuleResolutionErrorKind::ParseError { file } = kind {
            assert_eq!(file, PathBuf::from("tests/parser/issue-4126/invalid.rs"));
        } else {
            panic!("Expected parser error");
        }
    } else {
        panic!("Expected ModuleResolution operation error");
    }
}

fn assert_parser_error(filename: &str, exp_panic: bool) {
    let file = PathBuf::from(filename);
    let (config, operation, _) = read_config(&file);
    if let Err(OperationError::ParseError { input, is_panic }) =
        format_file(&file, operation, config)
    {
        assert_eq!(input.as_path().unwrap(), file);
        assert_eq!(is_panic, exp_panic);
    } else {
        panic!("Expected ParseError operation error");
    }
}

#[test]
fn parser_creation_errors_on_entry_new_parser_from_file_panic() {
    // See also https://github.com/rust-lang/rustfmt/issues/4418
    let filename = "tests/parser/issue_4418.rs";
    let should_panic = true;
    assert_parser_error(filename, should_panic);
}

#[test]
fn crate_parsing_errors_on_unclosed_delims() {
    // See also https://github.com/rust-lang/rustfmt/issues/4466
    let filename = "tests/parser/unclosed-delims/issue_4466.rs";
    let should_panic = false;
    assert_parser_error(filename, should_panic);
}
