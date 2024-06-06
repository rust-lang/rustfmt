use clap::Parser;
use std::env;
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

fn clone_git_repo(url: String, dest: String) {
    env::set_var("GIT_TERMINAL_PROMPT", "0");
    let mut git_cmd = Command::new("git");
    git_cmd.args(["clone", "--quiet", url, "--depth", "1", "dest"]);
    git_cmd.output().expect("failed to clone repository");
    let mut enter_dest_dir = Command::new("cd")
        .arg(dest)
        .output()
        .expect("failed to enter directory.");
}

fn main() {
    let args = CliInputs::parse();
    println!(
        "remote_repo_url: {:?}, feature_branch: {:?},
        optional_commit_hash: {:?}, optional_rustfmt_config: {:?}",
        args.remote_repo_url, args.feature_branch, args.commit_hash, args.rustfmt_config
    );
}
