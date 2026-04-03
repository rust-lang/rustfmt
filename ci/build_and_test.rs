use crate::common::run_command_with_env;

use std::collections::HashMap;

fn run_tests_in_dir(env: &HashMap<&str, &str>, dir: &str) -> Result<(), String> {
    run_command_with_env("cargo", &["build", "--locked"], dir, &env)?;
    run_command_with_env("cargo", &["test"], dir, &env)
}

pub fn runner() -> Result<(), String> {
    let env = HashMap::from([("RUSTFLAGS", "-D warnings"), ("RUSTFMT_CI", "1")]);

    // Print version information
    run_command_with_env("rustc", &["-Vv"], ".", &env)?;
    run_command_with_env("cargo", &["-v"], ".", &env)?;

    // Build and test main crate
    let options: &[&str] =
        if std::env::var("CFG_RELEASE_CHANNEL").is_ok_and(|value| value == "nightly") {
            &["build", "--locked", "--all-features"]
        } else {
            &["build", "--locked"]
        };
    run_command_with_env("cargo", options, ".", &env)?;
    run_command_with_env("cargo", &["test"], ".", &env)?;

    // Build and test config_proc_macro
    run_tests_in_dir(&env, "config_proc_macro")?;
    run_tests_in_dir(&env, "check_diff")?;

    Ok(())
}
