// Integration tests for cargo-fmt.

use indoc::panicdoc;
use std::env;
use std::path::Path;
use std::process::Command;

use rustfmt_config_proc_macro::rustfmt_only_ci_test;

/// Run the cargo-fmt executable and return its output.
fn cargo_fmt(args: &[&str]) -> (String, String) {
    cargo_fmt_with_env(args, &[])
}

fn cargo_fmt_with_env(args: &[&str], envs: &[(&str, &str)]) -> (String, String) {
    let mut bin_dir = env::current_exe().unwrap();
    bin_dir.pop(); // chop off test exe name

    // Handle Cargo's old and new filesystem layouts
    // * v1: `target/<profile>/deps/test-bin-[HASH][EXE]`
    // * v2: `target/<profile>/build/<pkgname>/[HASH]/out/test-bin-[HASH][EXE]`
    if bin_dir.ends_with("deps") {
        bin_dir.pop();
    } else if bin_dir.ends_with("out") {
        bin_dir.pop(); // chop off `out`
        bin_dir.pop(); // chop off `<hash>`
        bin_dir.pop(); // chop off `<pkgname>`
        bin_dir.pop(); // chop off `build`
    }
    let cmd = bin_dir.join(format!("cargo-fmt{}", env::consts::EXE_SUFFIX));

    // Ensure cargo-fmt runs the rustfmt binary from the local target dir.
    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.insert(0, bin_dir);
    let new_path = env::join_paths(paths).unwrap();

    match Command::new(&cmd)
        .args(args)
        .env("PATH", new_path)
        .envs(envs.iter().copied())
        .output()
    {
        Ok(output) => (
            String::from_utf8(output.stdout).expect("utf-8"),
            String::from_utf8(output.stderr).expect("utf-8"),
        ),
        Err(e) => panic!("failed to run `{cmd:?} {args:?}`: {e}"),
    }
}

macro_rules! assert_that {
    ($args:expr, $check:ident $check_args:tt) => {
        let (stdout, stderr) = cargo_fmt($args);
        if !stdout.$check$check_args {
            let args = $args;
            let check = stringify!($check);
            let check_args = stringify!($check_args);
            panicdoc! {"
                Output not expected for cargo-fmt {args:?}
                expected: {check}{check_args}
                actual stdout:
                {stdout}
                actual stderr:
                {stderr}",
            };
        }
    };
}

#[rustfmt_only_ci_test]
#[test]
fn version() {
    assert_that!(&["--version"], starts_with("rustfmt "));
    assert_that!(&["--version"], starts_with("rustfmt "));
    assert_that!(&["--", "-V"], starts_with("rustfmt "));
    assert_that!(&["--", "--version"], starts_with("rustfmt "));
}

#[rustfmt_only_ci_test]
#[test]
fn print_config() {
    assert_that!(
        &["--", "--print-config", "current", "."],
        contains("max_width = ")
    );
}

#[rustfmt_only_ci_test]
#[test]
fn rustfmt_help() {
    assert_that!(&["--", "--help"], contains("Format Rust code"));
    assert_that!(&["--", "-h"], contains("Format Rust code"));
    assert_that!(&["--", "--help=config"], contains("Configuration Options:"));
}

#[rustfmt_only_ci_test]
#[test]
fn cargo_fmt_out_of_line_test_modules() {
    // See also https://github.com/rust-lang/rustfmt/issues/5119
    let expected_modified_files = [
        "tests/mod-resolver/test-submodule-issue-5119/src/lib.rs",
        "tests/mod-resolver/test-submodule-issue-5119/tests/test1.rs",
        "tests/mod-resolver/test-submodule-issue-5119/tests/test1/sub1.rs",
        "tests/mod-resolver/test-submodule-issue-5119/tests/test1/sub2.rs",
        "tests/mod-resolver/test-submodule-issue-5119/tests/test1/sub3/sub4.rs",
    ];
    let args = [
        "-v",
        "--check",
        "--manifest-path",
        "tests/mod-resolver/test-submodule-issue-5119/Cargo.toml",
    ];
    let (stdout, _) = cargo_fmt(&args);
    for file in expected_modified_files {
        let path = Path::new(file).canonicalize().unwrap();
        assert!(stdout.contains(&format!("Diff in {}", path.display())));
    }
}

#[rustfmt_only_ci_test]
#[test]
fn cargo_fmt_emits_error_on_line_overflow_true() {
    // See also https://github.com/rust-lang/rustfmt/issues/3164
    let args = [
        "--check",
        "--manifest-path",
        "tests/cargo-fmt/source/issue_3164/Cargo.toml",
        "--",
        "--config",
        "error_on_line_overflow=true",
    ];

    let (_stdout, stderr) = cargo_fmt(&args);
    assert!(stderr.contains(
        "line formatted, but exceeded maximum width (maximum: 100 (see `max_width` option)"
    ))
}

// Win32 prohibits newline characters in file paths, while POSIX only discourages them (since
// POSIX.1-2024)
#[cfg(not(windows))]
#[rustfmt_only_ci_test]
#[test]
fn cargo_fmt_errors_when_passing_file_with_newline_to_at_file() {
    use std::fs;
    let tempdir = tempfile::tempdir().unwrap();
    let cargo_toml_path = tempdir.path().join("Cargo.toml");
    let cargo_toml_contents = r#"
[package]
name = "filename_handling"

[lib]
name = "filename_handling"
path = "src/file_with\nnewline.rs"
"#;

    let file_with_newline_path = tempdir.path().join(
        "src/file_with
newline.rs",
    );

    fs::create_dir_all(file_with_newline_path.parent().unwrap()).unwrap();
    fs::write(&cargo_toml_path, cargo_toml_contents).unwrap();
    fs::File::create(&file_with_newline_path).unwrap();

    let args = [
        "--check",
        "--verbose",
        "--manifest-path",
        cargo_toml_path.to_str().unwrap(),
    ];

    // set a low limit to force the exercising the '@path' logic
    let envs = [("_RUSTFMT_FILE_ARGS_LEN_MAX", "1")];
    let (stdout, stderr) = cargo_fmt_with_env(&args, &envs);

    assert!(
        stderr.contains(&format!(
            "refusing to pass file path containing newline as arg, file: {}",
            file_with_newline_path.canonicalize().unwrap().display(),
        )),
        "expected to see error message containing file {}\nstdout={stdout}\nstderr={stderr}",
        file_with_newline_path.display(),
    );
}

#[rustfmt_only_ci_test]
#[test]
fn cargo_fmt_arg_passing_via_at_path() {
    let expected_modified_files = [
        "tests/cargo-fmt/source/at_path_arg/src/lib.rs",
        "tests/cargo-fmt/source/at_path_arg/src/bin1.rs",
        "tests/cargo-fmt/source/at_path_arg/src/bin2.rs",
        "tests/cargo-fmt/source/at_path_arg/src/other.rs",
    ];
    let args = [
        "--check",
        "--verbose",
        "--manifest-path",
        "tests/cargo-fmt/source/at_path_arg/Cargo.toml",
    ];

    // set a low limit to force the exercising the '@path' logic
    let envs = [("_RUSTFMT_FILE_ARGS_LEN_MAX", "1")];
    let (stdout, stderr) = cargo_fmt_with_env(&args, &envs);

    for file in expected_modified_files {
        let path = Path::new(file).canonicalize().unwrap();
        assert!(
            stdout.contains(&format!("Diff in {}", path.display())),
            "Expected stdout to contain diff for file {}\nstdout={stdout}\nstderr={stderr}",
            path.display(),
        );
    }
    let rustfmt_args_line = stdout
        .lines()
        .find(|l| l.starts_with("rustfmt "))
        .expect("expected be a line logging args to rustfmt in stdout\nstdout={stdout}");

    assert!(
        rustfmt_args_line.contains(" @"),
        "expected an argument starting with '@' passed to rustfmt"
    );
}
