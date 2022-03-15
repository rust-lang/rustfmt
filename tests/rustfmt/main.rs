//! Integration tests for rustfmt.

use std::env;
use std::fs::remove_file;
use std::io::{Result, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};

use rustfmt_config_proc_macro::rustfmt_only_ci_test;

/// Run the rustfmt executable and return its output.
fn rustfmt(args: &[&str]) -> (String, String) {
    match rustfm_builder(|rustfmt| rustfmt.args(args).output()) {
        Ok(output) => (
            String::from_utf8(output.stdout).expect("utf-8"),
            String::from_utf8(output.stderr).expect("utf-8"),
        ),
        Err(e) => panic!("failed to run `rustfmt {:?}`: {}", args, e),
    }
}

/// Run the rustfmt executable and take input from stdin
fn rustfmt_std_input(args: &[&str], input: &str) -> (String, String) {
    let output = rustfm_builder(|cmd| {
        let mut rustfmt = cmd
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        rustfmt
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())?;
        rustfmt.wait_with_output()
    });
    match output {
        Ok(output) => (
            String::from_utf8(output.stdout).expect("utf-8"),
            String::from_utf8(output.stderr).expect("utf-8"),
        ),
        Err(e) => panic!("failed to run `rustfmt {:?}`: {}", args, e),
    }
}

fn rustfm_builder<F: Fn(&mut Command) -> Result<Output>>(f: F) -> Result<Output> {
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

    let mut rustfmt = Command::new(&cmd);
    f(rustfmt.env("PATH", new_path))
}

macro_rules! assert_that {
    ($args:expr, $($check:ident $check_args:tt)&&+) => {
        let (stdout, stderr) = rustfmt($args);
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
    };
}

#[rustfmt_only_ci_test]
#[test]
fn print_config() {
    assert_that!(
        &["--print-config", "unknown"],
        starts_with("Unknown print-config option")
    );
    assert_that!(&["--print-config", "default"], contains("max_width = 100"));
    assert_that!(&["--print-config", "minimal"], contains("PATH required"));
    assert_that!(
        &["--print-config", "minimal", "minimal-config"],
        contains("doesn't work with standard input.")
    );

    let (stdout, stderr) = rustfmt(&[
        "--print-config",
        "minimal",
        "minimal-config",
        "src/shape.rs",
    ]);
    assert!(
        Path::new("minimal-config").exists(),
        "stdout:\n{stdout}\nstderr:\n{stderr}"
    );
    remove_file("minimal-config").unwrap();
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
        contains("color = \"Never\"") && contains("edition = \"2018\"")
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
            && contains("format_strings = true")
    );
}

#[test]
fn rustfmt_usage_text() {
    let args = ["--help"];
    let (stdout, _) = rustfmt(&args);
    assert!(stdout.contains("Format Rust code\n\nusage: rustfmt [options] <file>..."));
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
    let (_stdout, stderr) = rustfmt(&args);
    assert!(stderr.contains(&error_message))
}

#[test]
fn mod_resolution_error_sibling_module_not_found() {
    let args = ["tests/mod-resolver/module-not-found/sibling_module/lib.rs"];
    let (_stdout, stderr) = rustfmt(&args);
    // Module resolution fails because we're unable to find `a.rs` in the same directory as lib.rs
    assert!(stderr.contains("a.rs does not exist"))
}

#[test]
fn mod_resolution_error_relative_module_not_found() {
    let args = ["tests/mod-resolver/module-not-found/relative_module/lib.rs"];
    let (_stdout, stderr) = rustfmt(&args);
    // The file `./a.rs` and directory `./a` both exist.
    // Module resolution fails because we're unable to find `./a/b.rs`
    #[cfg(not(windows))]
    assert!(stderr.contains("a/b.rs does not exist"));
    #[cfg(windows)]
    assert!(stderr.contains("a\\b.rs does not exist"));
}

#[test]
fn mod_resolution_error_path_attribute_does_not_exist() {
    let args = ["tests/mod-resolver/module-not-found/bad_path_attribute/lib.rs"];
    let (_stdout, stderr) = rustfmt(&args);
    // The path attribute points to a file that does not exist
    assert!(stderr.contains("does_not_exist.rs does not exist"));
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
    let files = [
        "tests/target/issue_5728.rs",
        "tests/target/issue_5729.rs",
        "tests/target/issue-5885.rs",
        "tests/target/issue_6082.rs",
        "tests/target/issue_6069.rs",
        "tests/target/issue-6105.rs",
    ];

    for file in files {
        let args = [file];
        let (_stdout, stderr) = rustfmt(&args);
        assert!(!stderr.contains("thread 'main' panicked"));
    }
}

#[test]
fn rustfmt_emits_error_when_control_brace_style_is_always_next_line() {
    // See also https://github.com/rust-lang/rustfmt/issues/5912
    let args = [
        "--config=color=Never",
        "--config",
        "control_brace_style=AlwaysNextLine",
        "--config",
        "match_arm_blocks=false",
        "tests/target/issue_5912.rs",
    ];

    let (_stdout, stderr) = rustfmt(&args);
    assert!(!stderr.contains("error[internal]: left behind trailing whitespace"))
}
mod rustfmt_stdin_formatting {
    use super::rustfmt_std_input;

    #[rustfmt::skip]
    #[test]
    fn changes_are_output_to_stdout() {
        let args = [];
        let source = "fn main     () {    println!(\"hello world!\"); }";
        let (stdout, _stderr) = rustfmt_std_input(&args, source);
        let expected_output =
r#"fn main() {
    println!("hello world!");
}"#;
        assert!(stdout.contains(expected_output))
    }

    #[test]
    fn properly_formatted_input_is_output_to_stdout_unchanged() {
        // NOTE: Technicallly a newline is added, but nothing meaningful is changed
        let args = [];
        let source = "fn main() {}";
        let (stdout, _stderr) = rustfmt_std_input(&args, source);
        assert!(stdout.trim_end() == source)
    }
}
