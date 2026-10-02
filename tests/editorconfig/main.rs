#![allow(unused)] // on stable

use rustfmt_config_proc_macro::nightly_only_test;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use tempfile::TempDir;

fn rustfmt_binary() -> PathBuf {
    let mut me = env::current_exe().expect("failed to get current executable");
    // Chop of the test name.
    me.pop();

    // Handle Cargo's old and new filesystem layouts
    // * v1: `target/<profile>/deps/test-bin-[HASH][EXE]`
    // * v2: `target/<profile>/build/<pkgname>/[HASH]/out/test-bin-[HASH][EXE]`
    if me.ends_with("deps") {
        // Chop off `deps`.
        me.pop();
    } else if me.ends_with("out") {
        // Chop off `out`.
        me.pop();
        // Chop off `<hash>`.
        me.pop();
        // Chop off `<pkgname>`.
        me.pop();
        // Chop off `build`.
        me.pop();
    }

    me.push("rustfmt");
    assert!(
        me.is_file() || me.with_extension("exe").is_file(),
        "{}",
        "no rustfmt bin, try running `cargo build --locked` or `cargo build --locked --release` \
        before testing"
    );
    me
}

/// Run rustfmt and return its output.
fn rustfmt(args: &[&str]) -> (String, String) {
    let rustfmt = rustfmt_binary();
    let cmd = rustfmt;

    match Command::new(&cmd).args(args).output() {
        Ok(output) => (
            String::from_utf8(output.stdout).expect("utf-8"),
            String::from_utf8(output.stderr).expect("utf-8"),
        ),
        Err(e) => panic!("failed to run `{cmd:?} {args:?}`: {e}"),
    }
}

#[nightly_only_test]
#[test]
fn integration() {
    let source_base_path = Path::new("tests/editorconfig/source");
    let target_base_path = Path::new("tests/editorconfig/target");
    for test_directory in source_base_path.read_dir().unwrap() {
        let source_directory = test_directory.unwrap().path();
        let target_directory = target_base_path.join(source_directory.file_name().unwrap());
        let temp_source_directory = TempDir::new().unwrap();
        let output_directory = temp_source_directory.path();
        for required_file in ["main.rs", ".editorconfig"] {
            fs::copy(
                source_directory.join(required_file),
                output_directory.join(required_file),
            )
            .unwrap();
        }
        let _ = rustfmt(&[
            "--unstable-features",
            "--use-editorconfig",
            output_directory.join("main.rs").to_string_lossy().as_ref(),
        ]);

        let reference_source = fs::read_to_string(target_directory.join("main.rs")).unwrap();
        let result_source = fs::read_to_string(output_directory.join("main.rs")).unwrap();
        assert_eq!(result_source, reference_source);
        drop(temp_source_directory);
    }
}

#[nightly_only_test]
#[test]
fn integration_errors() {
    let base_path = Path::new("tests/editorconfig/errors");
    for test_directory in base_path.read_dir().unwrap() {
        let test_directory = test_directory.unwrap().path();
        let (_, stderr) = rustfmt(&[
            "--unstable-features",
            "--use-editorconfig",
            "--check",
            test_directory.join("main.rs").to_string_lossy().as_ref(),
        ]);
        println!("{stderr}");
        let editorconfig = fs::read_to_string(test_directory.join(".editorconfig")).unwrap();
        let expected_error = editorconfig
            .lines()
            .next()
            .unwrap()
            .strip_prefix("# rustfmt-expect: ");
        if let Some(expected_error) = expected_error {
            println!("Checking contents of '{stderr}' for '{expected_error}'…");
            assert!(stderr.contains(expected_error));
        } else {
            assert!(stderr.is_empty());
        }
    }
}
