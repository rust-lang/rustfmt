use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub enum CheckDiffError {
    FailedGit(GitError),
    FailedCommand(String),
    FailedUtf8(String),
    FailedSourceBuild(String),
    FailedCopy(String),
    FailedCargoVersion(String),
    FailedVersioning(String),
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

pub fn copy_src_to_dst(src: &str, dst: &str) -> Result<(), CheckDiffError> {
    let Ok(_) = Command::new("cp").args([src, dst]).output() else {
        return Err(CheckDiffError::FailedCopy(
            "Error copying rustfmt release to destination".to_string(),
        ));
    };
    return Ok(());
}

pub fn build_rsfmt_from_src(ld_lib_path: &String) -> Result<(), CheckDiffError> {
    let Ok(_) = Command::new("cargo")
        .env("LD_LIB_PATH", ld_lib_path)
        .args(["build", "-q", "--release", "--bin", "rustfmt"])
        .output()
    else {
        return Err(CheckDiffError::FailedSourceBuild(
            "Error building rustfmt from source".to_string(),
        ));
    };
    return Ok(());
}

pub fn get_binary_version(binary: String) -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new(binary.as_str()).args(["--version"]).output() else {
        return Err(CheckDiffError::FailedVersioning(format!("Failed to get version for {}", binary)));
    };

    let Ok(binary_version) = String::from_utf8(command.stdout) else {
        return Err(CheckDiffError::FailedUtf8(
            "Error converting binary version to string".to_string(),
        ));
    };
    return Ok(binary_version);
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
) -> Result<Command, CheckDiffError> {
    const RUSTFMT_REPO: &str = "https://github.com/rust-lang/rustfmt.git";

    let _clone_git_result = clone_git_repo(RUSTFMT_REPO, dest)?;
    let _git_remote_add_result = git_remote_add(remote_repo_url.as_str())?;
    let _git_fetch_result = git_fetch(feature_branch.as_str())?;

    let cargo_version = get_cargo_version()?;
    info!("Compiling with {}", cargo_version);

    //Because we're building standalone binaries we need to set `LD_LIBRARY_PATH` so each
    // binary can find it's runtime dependencies.
    // See https://github.com/rust-lang/rustfmt/issues/5675
    // This will prepend the `LD_LIBRARY_PATH` for the master rustfmt binary
    let ld_lib_path = get_ld_lib_path()?;

    info!("Building rustfmt from source");
    let _build_from_src = build_rsfmt_from_src(&ld_lib_path)?;

    let rustfmt_binary = format!("{}/rustfmt", dest.display());
    let _cp = copy_src_to_dst("target/release/rustfmt", rustfmt_binary.as_str())?;

    let should_detach = commit_hash.is_some();
    let _ = git_switch(
        commit_hash.unwrap_or(feature_branch).as_str(),
        should_detach,
    );

    // This will prepend the `LD_LIBRARY_PATH` for the feature branch rustfmt binary.
    //  In most cases the `LD_LIBRARY_PATH` should be the same for both rustfmt binaries that we build
    //  in `compile_rustfmt`, however, there are scenarios where each binary has different runtime
    //  dependencies. For example, during subtree syncs we bump the nightly toolchain required to build
    //  rustfmt, and therefore the feature branch relies on a newer set of runtime dependencies.
    let ld_lib_path = get_ld_lib_path()?;
    info!("Building rustfmt from source");
    let _build_from_src = build_rsfmt_from_src(&ld_lib_path)?;
    let feature_binary = format!("{}/feature_rustfmt", dest.display());

    let _cp = copy_src_to_dst("target/release/rustfmt", rustfmt_binary.as_str())?;
    info!("\nRuntime dependencies for rustfmt -- LD_LIBRARY_PATH: {}", &ld_lib_path);

    let rustfmt_version = get_binary_version(rustfmt_binary)?;
    info!("\nRUSFMT_BIN {}\n", rustfmt_version);

    let feature_binary_version = get_binary_version(feature_binary)?;
    info!("FEATURE_BIN {}\n", feature_binary_version);

    let result = Command::new("ls");

    return Ok(result);
}
