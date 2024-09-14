use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use tracing::info;

pub enum CheckDiffError {
    FailedGit(GitError),
    FailedCommand(String),
    FailedUtf8(String),
    FailedSourceBuild(String),
    FailedCopy(String),
    FailedCargoVersion(String),
    FailedVersioning(String),
    FailedPathBuf(String),
    IO(std::io::Error),
}

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
    #[allow(dead_code)]
    fn run(&self, args: &[&str]) -> io::Result<Output> {
        Command::new(&self.binary_path)
            .env("LD_LIBRARY_PATH", &self.ld_library_path)
            .args(args)
            .output()
    }
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

pub fn git_switch(arg: &str, should_detach: bool) -> Result<(), GitError> {
    let detach_arg = if should_detach { "--detach" } else { "" };
    let git_cmd = Command::new("git").args([arg, detach_arg]).output()?;

    if !git_cmd.status.success() {
        let error = GitError::FailedSwitch {
            stdout: git_cmd.stdout,
            stderr: git_cmd.stderr,
        };
        return Err(error);
    }

    info!("Successfully switched to {}", arg);
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

pub fn get_ld_lib_path() -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("rustc").args(["--print", "sysroot"]).output() else {
        return Err(CheckDiffError::FailedCommand(
            "Error getting sysroot".to_string(),
        ));
    };

    let Ok(sysroot) = String::from_utf8(command.stdout) else {
        return Err(CheckDiffError::FailedUtf8(
            "Error converting sysroot to string".to_string(),
        ));
    };

    let ld_lib_path = format!("{}/lib", sysroot.trim_end());
    return Ok(ld_lib_path);
}

pub fn get_cargo_version() -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("cargo").args(["--version"]).output() else {
        return Err(CheckDiffError::FailedCargoVersion(
            "Failed to obtain cargo version".to_string(),
        ));
    };

    let Ok(cargo_version) = String::from_utf8(command.stdout) else {
        return Err(CheckDiffError::FailedUtf8(
            "Error converting cargo version to string".to_string(),
        ));
    };

    return Ok(cargo_version);
}

pub fn get_binary_version(
    binary: &PathBuf,
    ld_lib_path: &String,
) -> Result<String, CheckDiffError> {
    let binary_str = pathbuf_to_str(binary)?;
    let Ok(command) = Command::new(binary_str)
        .env("LD_LIB_PATH", ld_lib_path)
        .args(["--version"])
        .output()
    else {
        return Err(CheckDiffError::FailedVersioning(format!(
            "Failed to get version for {:?}",
            binary
        )));
    };

    let Ok(binary_version) = String::from_utf8(command.stdout) else {
        return Err(CheckDiffError::FailedUtf8(
            "Error converting binary version to string".to_string(),
        ));
    };
    return Ok(binary_version);
}

/// Obtains the ld_lib path and then builds rustfmt from source
/// If that operation succeeds, the source is then copied to the output path specified
pub fn build_rustfmt_from_src(output_path: &PathBuf) -> Result<RustfmtRunner, CheckDiffError> {
    //Because we're building standalone binaries we need to set `LD_LIBRARY_PATH` so each
    // binary can find it's runtime dependencies.
    // See https://github.com/rust-lang/rustfmt/issues/5675
    // This will prepend the `LD_LIBRARY_PATH` for the master rustfmt binary
    let ld_lib_path = get_ld_lib_path()?;

    info!("Building rustfmt from source");
    let Ok(_) = Command::new("cargo")
        .env("LD_LIB_PATH", &ld_lib_path.as_str())
        .args(["build", "-q", "--release", "--bin", "rustfmt"])
        .output()
    else {
        return Err(CheckDiffError::FailedSourceBuild(
            "Error building rustfmt from source".to_string(),
        ));
    };

    std::fs::copy("target/release/rustfmt", &output_path)?;

    return Ok(RustfmtRunner {
        ld_library_path: ld_lib_path,
        binary_path: output_path.to_path_buf(),
    });
}

fn pathbuf_to_str(pathbuf: &PathBuf) -> Result<&str, CheckDiffError> {
    let Some(result) = pathbuf.to_str() else {
        return Err(CheckDiffError::FailedPathBuf(format!(
            "Unable to convert {:?} to str",
            pathbuf
        )));
    };
    return Ok(result);
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
    git_remote_add(remote_repo_url.as_str())?;
    git_fetch(feature_branch.as_str())?;

    let cargo_version = get_cargo_version()?;
    info!("Compiling with {}", cargo_version);
    let rustfmt_binary = dest.join("/rustfmt");
    let src_runner = build_rustfmt_from_src(&rustfmt_binary)?;

    let should_detach = commit_hash.is_some();
    git_switch(
        commit_hash.unwrap_or(feature_branch).as_str(),
        should_detach,
    )?;
    let feature_binary = dest.join("/feature_rustfmt");

    let feature_runner = build_rustfmt_from_src(&feature_binary)?;

    info!(
        "\nRuntime dependencies for rustfmt -- LD_LIBRARY_PATH: {}",
        &feature_runner.ld_library_path
    );

    let rustfmt_version = get_binary_version(&rustfmt_binary, &feature_runner.ld_library_path)?;
    info!("\nRUSFMT_BIN {}\n", rustfmt_version);

    let feature_binary_version =
        get_binary_version(&feature_binary, &(feature_runner.ld_library_path))?;
    info!("FEATURE_BIN {}\n", feature_binary_version);

    return Ok(CheckDiffRunners {
        src_runner,
        feature_runner,
    });
}
