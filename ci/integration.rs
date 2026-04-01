use std::collections::HashMap;
use std::env::var;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

fn write_file(file_path: impl AsRef<Path>, content: &str) -> Result<(), String> {
    std::fs::write(&file_path, content).map_err(|error| {
        format!(
            "Failed to create empty `{}` file: {error:?}",
            file_path.as_ref().display(),
        )
    })
}

fn run_command_with_env<I, S>(
    bin: &str,
    args: I,
    current_dir: &str,
    env: &HashMap<&str, &str>,
) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let exit_status = Command::new(bin)
        .args(args)
        .envs(env)
        .current_dir(current_dir)
        .spawn()
        .map_err(|error| format!("Failed to spawn command `{bin}`: {error:?}"))?
        .wait()
        .map_err(|error| format!("Failed to wait command `{bin}`: {error:?}"))?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(format!("Command `{bin}` failed"))
    }
}

fn run_command<I, S>(bin: &str, args: I, current_dir: &str) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_command_with_env(bin, args, current_dir, &HashMap::new())
}

struct CommandOutput {
    output: String,
    exited_successfully: bool,
}

fn run_command_with_output_and_env<I, S>(
    bin: &str,
    args: I,
    current_dir: &str,
    env: &HashMap<&str, &str>,
) -> Result<CommandOutput, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let cmd_output = Command::new(bin)
        .args(args)
        .envs(env)
        .current_dir(current_dir)
        .output()
        .map_err(|error| format!("Failed to spawn command `{bin}`: {error:?}"))?;
    let mut output = String::from_utf8_lossy(&cmd_output.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&cmd_output.stderr));
    Ok(CommandOutput {
        output,
        exited_successfully: cmd_output.status.success(),
    })
}

fn run_command_with_output<I, S>(
    bin: &str,
    args: I,
    current_dir: &str,
) -> Result<CommandOutput, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_command_with_output_and_env(bin, args, current_dir, &HashMap::new())
}

// Checks that:
//
// * `cargo fmt --all` succeeds without any warnings or errors
// * `cargo fmt --all -- --check` after formatting returns success
// * `cargo test --all` still passes (formatting did not break the build)
fn check_fmt_with_all_tests(env: HashMap<&str, &str>, current_dir: &str) -> Result<(), String> {
    check_fmt_base("--all", env, current_dir)
}

// Checks that:
//
// * `cargo fmt --all` succeeds without any warnings or errors
// * `cargo fmt --all -- --check` after formatting returns success
// * `cargo test --lib` still passes (formatting did not break the build)
fn check_fmt_with_lib_tests(env: HashMap<&str, &str>, current_dir: &str) -> Result<(), String> {
    check_fmt_base("--lib", env, current_dir)
}

fn check_fmt_base(
    test_args: &str,
    env: HashMap<&str, &str>,
    current_dir: &str,
) -> Result<(), String> {
    fn check_output_does_not_contain(output: &str, needle: &str) -> Result<(), String> {
        if output.contains(needle) {
            Err(format!("`cargo fmt --all -v` contains `{needle}`"))
        } else {
            Ok(())
        }
    }

    let output =
        run_command_with_output_and_env("cargo", &["test", test_args], current_dir, &env)?.output;
    if ["build failed", "test result: FAILED."]
        .iter()
        .any(|needle| output.contains(needle))
    {
        println!("`cargo test {test_args}` failed: {output}");
        return Ok(());
    }

    let rustfmt_toml = Path::new(current_dir).join("rustfmt.toml");
    if !rustfmt_toml.is_file() {
        write_file(rustfmt_toml, "")?;
    }

    let output =
        run_command_with_output_and_env("cargo", &["fmt", "--all", "-v"], current_dir, &env)?;
    println!("{}", output.output);

    if !output.exited_successfully {
        return Err("`cargo fmt --all -v` failed".to_string());
    }

    let output = &output.output;
    check_output_does_not_contain(output, "internal error")?;
    check_output_does_not_contain(output, "internal compiler error")?;
    check_output_does_not_contain(output, "warning")?;
    check_output_does_not_contain(output, "Warning")?;

    let output = run_command_with_output_and_env(
        "cargo",
        &["fmt", "--all", "--", "--check"],
        current_dir,
        &env,
    )?;

    if !output.exited_successfully {
        return Err("`cargo fmt --all -- -v --check` failed".to_string());
    }
    let output = &output.output;
    if let Err(error) = write_file(Path::new(current_dir).join("rustfmt_check_output"), output) {
        println!("{output}");
        return Err(error);
    }

    // This command allows to ensure that no source file was modified while running the tests.
    run_command_with_env("cargo", &["test", test_args], current_dir, &env)
}

fn show_head(integration: &str) -> Result<(), String> {
    let head = run_command_with_output("git", &["rev-parse", "HEAD"], integration)?.output;
    println!("Head commit of {integration}: {head}");
    Ok(())
}

fn run_test<F: FnOnce(HashMap<&str, &str>, &str) -> Result<(), String>>(
    integration: &str,
    git_repository: String,
    env: HashMap<&str, &str>,
    test_fn: F,
) -> Result<(), String> {
    run_command_with_output("git", &["clone", "--depth=1", git_repository.as_str()], ".")?;
    show_head(integration)?;
    test_fn(env, integration)
}

fn runner() -> Result<(), String> {
    let integration = match var("INTEGRATION") {
        Ok(value) if !value.is_empty() => value,
        _ => {
            return Err("The INTEGRATION environment variable must be set.".into());
        }
    };

    run_command_with_env(
        "cargo",
        &["install", "--path", ".", "--force", "--locked"],
        ".",
        &HashMap::from([
            ("CFG_RELEASE", "nightly"),
            ("CFG_RELEASE_CHANNEL", "nightly"),
        ]),
    )?;

    println!("Integration tests for {integration}");

    run_command("cargo", &["fmt", "--", "--version"], ".")?;

    match integration.as_str() {
        "cargo" => run_test(
            &integration,
            format!("https://github.com/rust-lang/{integration}.git"),
            HashMap::from([("CFG_DISABLE_CROSS_TESTS", "1")]),
            check_fmt_with_all_tests,
        ),
        "crater" => run_test(
            &integration,
            format!("https://github.com/rust-lang/{integration}.git"),
            HashMap::new(),
            check_fmt_with_lib_tests,
        ),
        "bitflags" => run_test(
            &integration,
            format!("https://github.com/bitflags/{integration}.git"),
            HashMap::new(),
            check_fmt_with_all_tests,
        ),
        "tempdir" => run_test(
            &integration,
            format!("https://github.com/rust-lang-deprecated/{integration}.git"),
            HashMap::new(),
            check_fmt_with_all_tests,
        ),
        _ => run_test(
            &integration,
            format!("https://github.com/rust-lang/{integration}.git"),
            HashMap::new(),
            check_fmt_with_all_tests,
        ),
    }
}

fn main() {
    if let Err(error) = runner() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
