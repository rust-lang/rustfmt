//! Integration tests for rustfmt.

use std::env;
use std::fs::remove_file;
use std::path::Path;
use std::process::Command;

use rustfmt_config_proc_macro::rustfmt_only_ci_test;

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
        "stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
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

#[cfg(not(windows))]
mod load_config {
    use super::rustfmt;
    use std::path::Path;

    /// helper function to write a new rustfmt.toml to a file
    fn write_rustfmt_toml<P: AsRef<Path>>(path: P) {
        let dir_path = if path.as_ref().is_dir() {
            path.as_ref()
        } else {
            path.as_ref().parent().unwrap()
        };

        rustfmt(&[
            "--print-config=default",
            &dir_path.join("rustfmt.toml").display().to_string(),
        ]);
    }

    /// helper function to turn an AsRef<Path> into a String
    fn path_arg<P: AsRef<Path>>(path: P) -> String {
        path.as_ref().display().to_string()
    }

    macro_rules! assert_using_expected_config {
        ($file_path:expr) => {
            // case where we don't specify a config
            // No configs should exist in the file system when calling this case
            let (stdout, _stderr) = rustfmt(&["-v", &path_arg($file_path)]);
            assert!(!stdout.contains("rustfmt.toml"));
        };

        ($file_path:expr, $config_dir:expr) => {
            // case where we expect a config to be implicitly loaded
            let (stdout, _stderr) = rustfmt(&["-v", &path_arg($file_path)]);
            assert!(stdout.contains(&path_arg($config_dir)));
        };

        ($file_path:expr, "--config-path", $config_path:expr) => {
            // case where we explictly set a config and ensure it gets loaded
            let (stdout, _stderr) = rustfmt(&[
                "-v",
                "--config-path",
                &path_arg($config_path),
                &path_arg($file_path),
            ]);
            assert!(stdout.contains(&path_arg($config_path)));
        };
    }
    /// Ensure that we're loading the correct config when running rustfmt
    ///
    /// Configs are loaded in the following order:
    /// 1. Load configs from the `--config-path` option.
    /// 2. Travers the directory hierarchy looking for a config file
    /// 3. Check the user's `HOME` directory for a config file (could vary by platform)
    /// 4. Check the user's config directory for a config file (could vary by platform)
    ///
    /// When no configs are found rustfmt is run with just the default options.
    #[test]
    fn test_load_config_logic() {
        // Create a temporary directory and set it as the new $HOME.
        // This sets up a clean environment that we'll use to test the config loading logic
        let tmpdir = tempfile::tempdir().unwrap();
        let _home_env = tmp_env::set_var("HOME", tmpdir.as_ref());

        // Sanity check to make sure that we set the $HOME directory
        let home_dir = dirs::home_dir().unwrap();
        assert_eq!(tmpdir.as_ref(), home_dir.as_path());

        // Create a working directory nested a few levels deep inside the temporary $HOME.
        // We want a few directory levels so we can properly test case #2 listed above.
        // Set the current working directory to the new path so we don't pick up any rustfmt.toml
        // files defined outside our clean environment.
        let work_dir = home_dir.join("path").join("to").join("file");
        std::fs::create_dir_all(&work_dir).unwrap();
        let _current_dir = tmp_env::set_current_dir(&work_dir).unwrap();

        // Set up the user's config directory
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("rustfmt");
        std::fs::create_dir_all(&config_dir).unwrap();

        // Write a hello world file used for formatting checks in the working directory
        let file_path = work_dir.join("test.rs");
        std::fs::write(
            &file_path,
            "fn main() {\n    println!(\"Hello world!\");\n}",
        )
        .unwrap();

        // 1. Run rustfmt and make sure we don't load any configs
        assert_using_expected_config!(&file_path);

        // Write a rustfmt.toml to the users config directory
        // 2. Run rustfmt and make sure we load the config from the user's config dir.
        //    Sinces no other configs exist this one should be used.
        write_rustfmt_toml(&config_dir);
        assert_using_expected_config!(&file_path, &config_dir);

        // Write a rustfmt.toml to the users $HOME directory
        // 3. Run rustmft and make sure we load the config from the user's $HOME dir
        //    Configs in the $HOME dir take precedence over those in the config dir
        write_rustfmt_toml(&home_dir);
        assert_using_expected_config!(&file_path, &home_dir);

        // write a rustfmt.toml to some directory in the `work_dir` hierarchy.
        // 4. Run rustfmt and make sure we load the config from the file hierarcy.
        //    Configs found in the file hierarcy take precedence to those in $HOME and the config dir.
        let config_along_path = work_dir.parent().unwrap().parent().unwrap();
        write_rustfmt_toml(&config_along_path);
        assert_using_expected_config!(&file_path, &config_along_path);

        // write a rustfmt.toml outside the working directory hierarchy.
        // This ensures it isn't automatically picked up.
        // 5. run rustfmt and explicitly set the `--config-path` option to this config file.
        //    Configs that are explicity set take precedence over all other configs.
        let explicit_config_path = home_dir.join("new").join("config").join("path");
        std::fs::create_dir_all(&explicit_config_path).unwrap();
        write_rustfmt_toml(&explicit_config_path);
        assert_using_expected_config!(&file_path, "--config-path", &explicit_config_path);
    }
}
