use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::process::Command;
use tracing::info;

enum GitCommand {
    Clone,
    RemoteAdd,
    Fetch,
}

pub enum GitError {
    FailedClone { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedRemoteAdd { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedFetch { stdout: Vec<u8>, stderr: Vec<u8> },
    IO(std::io::Error),
}

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError::IO(error)
    }
}
/// Runs the git command with specified args, env vars,
/// error and log messages
fn git_runner(
    args: Vec<&str>,
    envs: HashMap<&str, &str>,
    command: GitCommand,
    log_message: &str,
) -> Result<(), GitError> {
    let git_cmd = Command::new("git").envs(&envs).args(args).output()?;
    // if the git command does not return successfully,
    //  any command on the repo will fail. So fail fast.
    if !git_cmd.status.success() {
        let git_error = match command {
            GitCommand::Clone => GitError::FailedClone {
                stdout: git_cmd.stdout,
                stderr: git_cmd.stderr,
            },
            GitCommand::Fetch => GitError::FailedFetch {
                stdout: git_cmd.stdout,
                stderr: git_cmd.stderr,
            },
            GitCommand::RemoteAdd => GitError::FailedRemoteAdd {
                stdout: git_cmd.stdout,
                stderr: git_cmd.stderr,
            },
        };

        return Err(git_error);
    }
    info!("{}", log_message);
    return Ok(());
}

/// Clone a git repository
///
/// Parameters:
/// url: git clone url
/// dest: directory where the repo should be cloned
pub fn clone_git_repo(url: &str, dest: &Path) -> Result<(), GitError> {
    let args = [
        "clone",
        "--quiet",
        url,
        "--depth",
        "1",
        dest.to_str().unwrap(),
    ]
    .to_vec();
    let env_vars = HashMap::from([("GIT_TERMINAL_PROMPT", "0")]);
    let log_message = "Successfully clone repository.";
    git_runner(args, env_vars, GitCommand::Clone, log_message)
}

pub fn git_remote_add(url: &str) -> Result<(), GitError> {
    let args = ["remote", "add", "feature", url].to_vec();
    let log_message = "Successfully added remote.";
    git_runner(args, HashMap::new(), GitCommand::RemoteAdd, log_message)
}

pub fn git_fetch(branch_name: &str) -> Result<(), GitError> {
    let args = ["fetch", "feature", branch_name].to_vec();
    let log_message = "Successfully fetched.";
    git_runner(args, HashMap::new(), GitCommand::Fetch, log_message)
}
