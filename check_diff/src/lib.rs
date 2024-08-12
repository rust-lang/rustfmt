use git::clone_git_repo;
use git::git_fetch;
use git::git_remote_add;
use git::GitError;
use std::env;
use std::io;
use std::path::Path;
use structs::CliInputs;
use tracing::info;

pub mod git;
pub mod structs;

const RUSTFMT_REPO: &str = "https://github.com/rust-lang/rustfmt.git";

pub fn change_directory_to_path(dest: &Path) -> io::Result<()> {
    let dest_path = Path::new(&dest);
    env::set_current_dir(&dest_path)?;
    info!(
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
    return Ok(());
}

// Compiles and produces two rustfmt binaries.
// One for the current master, and another for the feature branch
// Parameters:
// dest: Directory where rustfmt will be cloned
pub fn compile_rustfmt(dest: &Path, inputs: CliInputs) -> Result<(), GitError> {
    let clone_repo_result = clone_git_repo(RUSTFMT_REPO, dest);

    if clone_repo_result.is_err() {
        return clone_repo_result;
    }

    let remote_add_result = git_remote_add(inputs.remote_repo_url.as_str());
    if remote_add_result.is_err() {
        return remote_add_result;
    }

    let fetch_result = git_fetch(inputs.feature_branch.as_str());
    if fetch_result.is_err() {
        return fetch_result;
    }

    let cargo_version = env!("CARGO_PKG_VERSION");
    info!("Compiling with {}", cargo_version);

    return Ok(());
}
