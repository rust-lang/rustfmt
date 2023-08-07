//! Integration tests for rustfmt.

use std::env;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

use rustfmt_config_proc_macro::{nightly_only_test, rustfmt_only_ci_test, stable_only_test};

/// Run the rustfmt executable and return its output.
fn rustfmt(args: &[&str]) -> (String, String, i32) {
    let mut bin_dir = env::current_exe().unwrap();
    bin_dir.pop(); // chop off test exe name
    if bin_dir.ends_with("deps") {
        bin_dir.pop();
    }
    let cmd = bin_dir.join(format!("rustfmt{}", env::consts::EXE_SUFFIX));

    // Ensure the rustfmt binary runs from the local target dir.
    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.insert(0, bin_dir);
    let new_path = env::join_paths(paths).unwrap();

    match Command::new(&cmd).args(args).env("PATH", new_path).output() {
        Ok(output) => (
            String::from_utf8(output.stdout).expect("utf-8"),
            String::from_utf8(output.stderr).expect("utf-8"),
            output.status.code().expect("should have exit status code"),
        ),
        Err(e) => panic!("failed to run `{:?} {:?}`: {}", cmd, args, e),
    }
}

macro_rules! assert_that {
    ($args:expr, $($check:ident $check_args:tt)&&+, $exit_code:expr) => {
        let (stdout, stderr, exit_code) = rustfmt($args);
        if $(!stdout.$check$check_args && !stderr.$check$check_args)||* {
            panic!(
                "Output not expected for rustfmt {:?}\n\
                 expected: {}\n\
                 actual stdout:\n{}\n\
                 actual stderr:\n{}",
                $args,
                stringify!($( $check$check_args )&&*),
                stdout,
                stderr
            );
        }
        if $exit_code != exit_code {
            panic!(
                "rustfmt exited with a status code of {}. The expected status code was {}",
                exit_code,
                $exit_code,
            );
        }
    };
}

#[rustfmt_only_ci_test]
#[test]
fn print_config() {
    assert_that!(
        &["--print-config", "unknown"],
        starts_with("Unknown print-config option"),
        1
    );
    assert_that!(
        &["--print-config", "default"],
        contains("max_width = 100"),
        0
    );
    assert_that!(&["--print-config", "minimal"], contains("PATH required"), 0);
    assert_that!(
        &["--print-config", "minimal", "minimal-config"],
        contains("doesn't work with standard input."),
        1
    );

    let (stdout, stderr, exit_code) = rustfmt(&[
        "--print-config",
        "minimal",
        "minimal-config",
        "src/shape.rs",
    ]);
    assert!(
        Path::new("minimal-config").exists(),
        "stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    remove_file("minimal-config").unwrap();
    assert_eq!(exit_code, 0);
}

#[rustfmt_only_ci_test]
#[test]
fn inline_config() {
    // single invocation
    assert_that!(
        &[
            "--print-config",
            "current",
            ".",
            "--config=color=Never,edition=2018"
        ],
        contains("color = \"Never\"") && contains("edition = \"2018\""),
        0
    );

    // multiple overriding invocations
    assert_that!(
        &[
            "--print-config",
            "current",
            ".",
            "--config",
            "color=never,edition=2018",
            "--config",
            "color=always,format_strings=true"
        ],
        contains("color = \"Always\"")
            && contains("edition = \"2018\"")
            && contains("format_strings = true"),
        0
    );
}

// `abort_on_unrecognized_options = true` causes stable rustfmt to exit early when
// unstable options are set in the configuration toml file
#[stable_only_test]
#[test]
fn abort_when_using_unstable_options_in_toml_config_while_on_stable_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/some_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=true",
            "src/bin/main.rs",
        ],
        contains("Can't set nightly options when using stable rustfmt")
            && contains("`wrap_comments = true`")
            && contains("`unstable_features = true`"),
        1
    );
}

// `abort_on_unrecognized_options = true` has no impact when unstable options are passed
// via the comand line. The option only applies to toml configuration.
#[stable_only_test]
#[test]
fn do_not_abort_when_using_unstable_options_from_command_line_when_on_stable_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/no_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=true,wrap_comments=true",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = false` causes stable rustfmt to display warnings when
// unstable options are set in the configuration toml file
#[stable_only_test]
#[test]
fn warn_when_using_unstable_options_in_toml_file_when_on_stable_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/some_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=false",
            "src/bin/main.rs",
        ],
        contains("Warning: can't set `wrap_comments = true`")
            && contains("Warning: can't set `unstable_features = true`"),
        0
    );
}

// `abort_on_unrecognized_options = false` has no impact when unstable options are passed
// via the comand line. The option only applies to toml configuration.
#[stable_only_test]
#[test]
fn do_not_warn_when_using_unstable_options_from_command_line_when_on_stable_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/no_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=false,wrap_comments=true",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = true` does nothing when no unstable options are used
#[stable_only_test]
#[test]
fn do_not_abort_when_only_using_stable_options_in_toml_file_and_cli_when_on_stable_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/no_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=true,max_width=100",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = true` doesn't affect nightly rustfmt
#[nightly_only_test]
#[test]
fn ignore_abort_option_when_using_unstable_options_in_toml_file_while_on_nightly_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/some_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=true",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = true` doesn't affect nightly rustfmt
#[nightly_only_test]
#[test]
fn ignore_abort_option_when_using_unstable_options_from_command_line_when_on_nightly_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config",
            "abort_on_unrecognised_options=true,wrap_comments=true",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = false` doesn't affect nightly rustfmt
#[nightly_only_test]
#[test]
fn warn_when_using_unstable_options_in_toml_file_when_on_nightly_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config-path",
            "tests/config/some_unstable_options.toml",
            "--config",
            "abort_on_unrecognised_options=false",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

// `abort_on_unrecognized_options = false` doesn't affect nightly rustfmt
#[nightly_only_test]
#[test]
fn warn_when_using_unstable_options_from_command_line_when_on_nightly_rustfmt() {
    assert_that!(
        &[
            "--check",
            "--config",
            "abort_on_unrecognised_options=false,wrap_comments=true",
            "src/bin/main.rs",
        ],
        is_empty(),
        0
    );
}

#[test]
fn rustfmt_usage_text() {
    let args = ["--help"];
    let (stdout, _stderr, code) = rustfmt(&args);
    assert!(stdout.contains("Format Rust code\n\nusage: rustfmt [options] <file>..."));
    assert_eq!(code, 0);
}

#[test]
fn mod_resolution_error_multiple_candidate_files() {
    // See also https://github.com/rust-lang/rustfmt/issues/5167
    let default_path = Path::new("tests/mod-resolver/issue-5167/src/a.rs");
    let secondary_path = Path::new("tests/mod-resolver/issue-5167/src/a/mod.rs");
    let error_message = format!(
        "file for module found at both {:?} and {:?}",
        default_path.canonicalize().unwrap(),
        secondary_path.canonicalize().unwrap(),
    );

    let args = ["tests/mod-resolver/issue-5167/src/lib.rs"];
    let (_stdout, stderr, code) = rustfmt(&args);
    assert!(stderr.contains(&error_message));
    assert_eq!(code, 1);
}

#[test]
fn mod_resolution_error_sibling_module_not_found() {
    let args = ["tests/mod-resolver/module-not-found/sibling_module/lib.rs"];
    let (_stdout, stderr, code) = rustfmt(&args);
    // Module resolution fails because we're unable to find `a.rs` in the same directory as lib.rs
    assert!(stderr.contains("a.rs does not exist"));
    assert_eq!(code, 1);
}

#[test]
fn mod_resolution_error_relative_module_not_found() {
    let args = ["tests/mod-resolver/module-not-found/relative_module/lib.rs"];
    let (_stdout, stderr, code) = rustfmt(&args);
    // The file `./a.rs` and directory `./a` both exist.
    // Module resolution fails because we're unable to find `./a/b.rs`
    #[cfg(not(windows))]
    assert!(stderr.contains("a/b.rs does not exist"));
    #[cfg(windows)]
    assert!(stderr.contains("a\\b.rs does not exist"));
    assert_eq!(code, 1);
}

#[test]
fn mod_resolution_error_path_attribute_does_not_exist() {
    let args = ["tests/mod-resolver/module-not-found/bad_path_attribute/lib.rs"];
    let (_stdout, stderr, code) = rustfmt(&args);
    // The path attribute points to a file that does not exist
    assert!(stderr.contains("does_not_exist.rs does not exist"));
    assert_eq!(code, 1);
}

#[test]
fn rustfmt_emits_error_on_line_overflow_true() {
    // See also https://github.com/rust-lang/rustfmt/issues/3164
    let args = [
        "--config",
        "error_on_line_overflow=true",
        "tests/cargo-fmt/source/issue_3164/src/main.rs",
    ];

    let (_stdout, stderr) = rustfmt(&args);
    assert!(stderr.contains(
        "line formatted, but exceeded maximum width (maximum: 100 (see `max_width` option)"
    ))
}

#[test]
#[allow(non_snake_case)]
fn dont_emit_ICE() {
    let files = ["tests/target/issue_5728.rs", "tests/target/issue_5729.rs"];

    for file in files {
        let args = [file];
        let (_stdout, stderr) = rustfmt(&args);
        assert!(!stderr.contains("thread 'main' panicked"));
    }
}
