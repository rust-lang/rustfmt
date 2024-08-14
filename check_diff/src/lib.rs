use std::env;
use std::io;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub enum GitError {
    FailedClone { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedRemoteAdd { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedFetch { stdout: Vec<u8>, stderr: Vec<u8> },
    IO(std::io::Error),
}

const RUSTFMT_REPO: &str = "https://github.com/rust-lang/rustfmt.git";

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError::IO(error)
    }
}

/// Clone a git repository
///
/// Parameters:
/// url: git clone url
/// dest: directory where the repo should be cloned
pub fn clone_git_repo(url: &str, dest: &Path) -> Result<(), GitError> {
    let git_cmd = Command::new("git")
        .env("GIT_TERMINAL_PROMPT", "0")
        .args([
            "clone",
            "--quiet",
            url,
            "--depth",
            "1",
            dest.to_str().unwrap(),
        ])
        .output()?;

    // if the git command does not return successfully,
    // any command on the repo will fail. So fail fast.
    if !git_cmd.status.success() {
        let error = GitError::FailedClone {
            stdout: git_cmd.stdout,
            stderr: git_cmd.stderr,
        };
        return Err(error);
    }

    info!("Successfully clone repository.");
    return Ok(());
}

pub fn git_remote_add(url: &str) -> Result<(), GitError> {
    let git_cmd = Command::new("git")
        .args(["remote", "add", "feature", url])
        .output()?;

    // if the git command does not return successfully,
    // any command on the repo will fail. So fail fast.
    if !git_cmd.status.success() {
        let error = GitError::FailedRemoteAdd {
            stdout: git_cmd.stdout,
            stderr: git_cmd.stderr,
        };
        return Err(error);
    }

    info!("Successfully added remote.");
    return Ok(());
}

pub fn git_fetch(branch_name: &str) -> Result<(), GitError> {
    let git_cmd = Command::new("git")
        .args(["fetch", "feature", branch_name])
        .output()?;

    // if the git command does not return successfully,
    // any command on the repo will fail. So fail fast.
    if !git_cmd.status.success() {
        let error = GitError::FailedFetch {
            stdout: git_cmd.stdout,
            stderr: git_cmd.stderr,
        };
        return Err(error);
    }

    info!("Successfully fetched.");
    return Ok(());
}

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
pub fn compile_rustfmt(
    dest: &Path,
    remote_repo_url: String,
    feature_branch: String,
    rustfmt_config: Option<Vec<String>>,
) -> io::Result<Command> {
    let Ok(_) = clone_git_repo(RUSTFMT_REPO, dest) else {
        return Err(Error::other("Error cloning repo while compiling rustfmt"));
    };

    let Ok(_) = git_remote_add(remote_repo_url.as_str()) else {
        return Err(Error::other(format!(
            "Error adding remote from {} while compiling rustfmt",
            remote_repo_url
        )));
    };

    let Ok(_) = git_fetch(feature_branch.as_str()) else {
        return Err(Error::other(format!(
            "Error fetching from {} while compiling rustfmt",
            feature_branch
        )));
    };

    let cargo_version = env!("CARGO_PKG_VERSION");
    info!("Compiling with {}", cargo_version);

    //Because we're building standalone binaries we need to set `LD_LIBRARY_PATH` so each
    // binary can find it's runtime dependencies.
    // See https://github.com/rust-lang/rustfmt/issues/5675
    // This will prepend the `LD_LIBRARY_PATH` for the master rustfmt binary

    let Ok(command) = std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
    else {
        return Err(Error::other("Error getting sysroot"));
    };

    let Ok(sysroot) = String::from_utf8(command.stdout) else {
        return Err(Error::other("Error converting sysroot to string"));
    };

    let ld_lib_path = format!("{}/lib", sysroot.trim_end());
    env::set_var("LD_LIBRARY_PATH", ld_lib_path);
    info!("Building rustfmt from scratch");

    let result = Command::new("ls");

    return Ok(result);
}
