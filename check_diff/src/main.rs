use clap::Parser;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Inputs for the check_diff script
#[derive(Parser)]
struct CliInputs {
    /// Git url of a rustfmt fork to compare against the latest master rustfmt
    remote_repo_url: String,
    /// Name of the feature branch on the forked repo
    feature_branch: String,
    /// Optional commit hash from the feature branch
    #[arg(short, long)]
    commit_hash: Option<String>,
    /// Optional comma separated list of rustfmt config options to
    /// pass when running the feature branch
    #[arg(value_delimiter = ',', short, long, num_args = 1..)]
    rustfmt_config: Option<Vec<String>>,
}

/// Clone a git repository and cd into it.
///
/// Parameters:
/// url: git clone url
/// dest: directory where the repo should be cloned
pub fn clone_git_repo(url: &str, dest: &Path) {
    env::set_var("GIT_TERMINAL_PROMPT", "0");
    let git_cmd = Command::new("git")
        .args([
            "clone",
            "--quiet",
            url,
            "--depth",
            "1",
            dest.to_str().unwrap(),
        ])
        .output()
        .expect("failed to clone repository");
    // if the git command does not return successfully,
    // cd command will not work as well. So fail fast.
    if !git_cmd.status.success() {
        io::stdout().write_all(&git_cmd.stdout).unwrap();
        io::stderr().write_all(&git_cmd.stderr).unwrap();
        return;
    }
    println!("Successfully clone repository. Entering repository");

    let dest_path = Path::new(&dest);
    let _ = env::set_current_dir(&dest_path).is_ok();
    println!(
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
}

fn main() {
    let _args = CliInputs::parse();
}
