use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::Utf8Error;
use tracing::info;

pub enum CheckDiffError {
    /// Git related errors
    FailedGit(GitError),
    /// Error for generic commands
    FailedCommand(&'static str),
    /// UTF8 related errors
    FailedUtf8(Utf8Error),
    /// Error for building rustfmt from source
    FailedSourceBuild(&'static str),
    /// Error when obtaining binary version
    FailedBinaryVersioning(PathBuf),
    /// Error when obtaining cargo version
    FailedCargoVersion(&'static str),
    IO(std::io::Error),
}

impl From<io::Error> for CheckDiffError {
    fn from(error: io::Error) -> Self {
        CheckDiffError::IO(error)
    }
}

impl From<GitError> for CheckDiffError {
    fn from(error: GitError) -> Self {
        CheckDiffError::FailedGit(error)
    }
}

impl From<Utf8Error> for CheckDiffError {
    fn from(error: Utf8Error) -> Self {
        CheckDiffError::FailedUtf8(error)
    }
}

pub enum GitError {
    FailedClone { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedRemoteAdd { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedFetch { stdout: Vec<u8>, stderr: Vec<u8> },
    FailedSwitch { stdout: Vec<u8>, stderr: Vec<u8> },
    IO(std::io::Error),
}

impl From<io::Error> for GitError {
    fn from(error: io::Error) -> Self {
        GitError::IO(error)
    }
}

// will be used in future PRs, just added to make the compiler happy
#[allow(dead_code)]
pub struct CheckDiffRunners {
    feature_runner: RustfmtRunner,
    src_runner: RustfmtRunner,
}

pub struct RustfmtRunner {
    ld_library_path: String,
    binary_path: PathBuf,
}

impl RustfmtRunner {
    fn get_binary_version(&self) -> Result<String, CheckDiffError> {
        let Ok(command) = Command::new(&self.binary_path)
            .env("LD_LIBRARY_PATH", &self.ld_library_path)
            .args(["--version"])
            .output()
        else {
            return Err(CheckDiffError::FailedBinaryVersioning(
                self.binary_path.clone(),
            ));
        };

        let binary_version = std::str::from_utf8(&command.stdout)?.trim();
        return Ok(binary_version.to_string());
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

    info!("Successfully added remote: {url}");
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

    info!("Successfully fetched: {branch_name}");
    return Ok(());
}

pub fn git_switch(git_ref: &str, should_detach: bool) -> Result<(), GitError> {
    let detach_arg = if should_detach { "--detach" } else { "" };
    let args = ["switch", git_ref, detach_arg];
    let output = Command::new("git")
        .args(args.iter().filter(|arg| !arg.is_empty()))
        .output()?;
    if !output.status.success() {
        tracing::error!("Git switch failed: {output:?}");
        let error = GitError::FailedSwitch {
            stdout: output.stdout,
            stderr: output.stderr,
        };
        return Err(error);
    }
    info!("Successfully switched to {git_ref}");
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

pub fn get_ld_library_path() -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("rustc").args(["--print", "sysroot"]).output() else {
        return Err(CheckDiffError::FailedCommand("Error getting sysroot"));
    };
    let sysroot = std::str::from_utf8(&command.stdout)?.trim_end();
    let ld_lib_path = format!("{}/lib", sysroot);
    return Ok(ld_lib_path);
}

pub fn get_cargo_version() -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("cargo").args(["--version"]).output() else {
        return Err(CheckDiffError::FailedCargoVersion(
            "Failed to obtain cargo version",
        ));
    };

    let cargo_version = std::str::from_utf8(&command.stdout)?.trim_end();
    return Ok(cargo_version.to_string());
}

/// Obtains the ld_lib path and then builds rustfmt from source
/// If that operation succeeds, the source is then copied to the output path specified
pub fn build_rustfmt_from_src(binary_path: PathBuf) -> Result<RustfmtRunner, CheckDiffError> {
    //Because we're building standalone binaries we need to set `LD_LIBRARY_PATH` so each
    // binary can find it's runtime dependencies.
    // See https://github.com/rust-lang/rustfmt/issues/5675
    // This will prepend the `LD_LIBRARY_PATH` for the master rustfmt binary
    let ld_lib_path = get_ld_library_path()?;

    info!("Building rustfmt from source");
    let Ok(_) = Command::new("cargo")
        .args(["build", "-q", "--release", "--bin", "rustfmt"])
        .output()
    else {
        return Err(CheckDiffError::FailedSourceBuild(
            "Error building rustfmt from source",
        ));
    };

    std::fs::copy("target/release/rustfmt", &binary_path)?;

    return Ok(RustfmtRunner {
        ld_library_path: ld_lib_path,
        binary_path,
    });
}

// Compiles and produces two rustfmt binaries.
// One for the current master, and another for the feature branch
// Parameters:
// dest: Directory where rustfmt will be cloned
pub fn compile_rustfmt(
    dest: &Path,
    remote_repo_url: String,
    feature_branch: String,
    commit_hash: Option<String>,
) -> Result<CheckDiffRunners, CheckDiffError> {
    const RUSTFMT_REPO: &str = "https://github.com/rust-lang/rustfmt.git";

    clone_git_repo(RUSTFMT_REPO, dest)?;
    change_directory_to_path(dest)?;
    git_remote_add(remote_repo_url.as_str())?;
    git_fetch(feature_branch.as_str())?;

    let cargo_version = get_cargo_version()?;
    info!("Compiling with {}", cargo_version);
    let src_runner = build_rustfmt_from_src(dest.join("src_rustfmt"))?;
    let should_detach = commit_hash.is_some();
    git_switch(
        commit_hash.unwrap_or(feature_branch).as_str(),
        should_detach,
    )?;

    let feature_runner = build_rustfmt_from_src(dest.join("feature_rustfmt"))?;
    info!("RUSFMT_BIN {}", src_runner.get_binary_version()?);
    info!(
        "Runtime dependencies for (src) rustfmt -- LD_LIBRARY_PATH: {}",
        src_runner.ld_library_path
    );
    info!("FEATURE_BIN {}", feature_runner.get_binary_version()?);
    info!(
        "Runtime dependencies for (feature) rustfmt -- LD_LIBRARY_PATH: {}",
        feature_runner.ld_library_path
    );

    return Ok(CheckDiffRunners {
        src_runner,
        feature_runner,
    });
}
