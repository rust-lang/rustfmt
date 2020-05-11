//! Integration tests for rustfmt.

use std::env;
use std::process::Command;

/// Run the rustfmt executable and return its output.
fn rustfmt(args: &[&str]) -> (String, String) {
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
        ),
        Err(e) => panic!("failed to run `{:?} {:?}`: {}", cmd, args, e),
    }
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

#[ignore]
#[test]
fn print_config() {
    assert_that!(
        &["--print-config", "unknown"],
        starts_with("error: Invalid value for '--print-config")
    );
    assert_that!(&["--print-config", "default"], contains("max_width = 100"));
    assert_that!(&["--print-config", "current"], contains("max_width = 100"));
    assert_that!(
        &["--config", "max_width=120", "--print-config", "current"],
        contains("max_width = 120")
    );
    assert_that!(
        &["--config", "max_width=120", "--print-config", "minimal"],
        contains("max_width = 120")
    );
    assert_that!(
        &["--config", "max_width=100", "--print-config", "minimal"],
        eq("")
    );
}

#[ignore]
#[test]
fn inline_config() {
    // single invocation
    assert_that!(
        &[
            "--print-config",
            "current",
            ".",
            "--config=max_width=50,edition=2018"
        ],
        contains("max_width = 50") && contains("edition = \"2018\"")
    );

    // multiple overriding invocations
    assert_that!(
        &[
            "--print-config",
            "current",
            ".",
            "--config",
            "max_width=80,edition=2018",
            "--config",
            "max_width=60,format_strings=true"
        ],
        contains("max_width = 60")
            && contains("edition = \"2018\"")
            && contains("format_strings = true")
    );
}
