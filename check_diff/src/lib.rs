use std::env;
use std::fmt::{Debug, Display};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::{FromStr, Utf8Error};
use tracing::{debug, error, info, trace};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub enum Edition {
    /// rust edition 2015
    Edition2015,
    /// rust edition 2018
    Edition2018,
    /// rust edition 2021
    Edition2021,
    /// rust edition 2024
    Edition2024,
}

impl Edition {
    fn as_str(&self) -> &str {
        match self {
            Edition::Edition2015 => "2015",
            Edition::Edition2018 => "2018",
            Edition::Edition2021 => "2021",
            Edition::Edition2024 => "2024",
        }
    }
}

impl FromStr for Edition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2015" => Ok(Edition::Edition2015),
            "2018" => Ok(Edition::Edition2018),
            "2021" => Ok(Edition::Edition2021),
            "2024" => Ok(Edition::Edition2024),
            _ => Err(format!("Invalid rust edition {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StyleEdition {
    // rustfmt style_edition 2021. Also equivaluent to 2015 and 2018.
    Edition2021,
    // rustfmt style_edition 2024
    Edition2024,
}

impl StyleEdition {
    fn as_str(&self) -> &str {
        match self {
            StyleEdition::Edition2021 => "2021",
            StyleEdition::Edition2024 => "2024",
        }
    }
}

impl FromStr for StyleEdition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2015" => Ok(StyleEdition::Edition2021),
            "2018" => Ok(StyleEdition::Edition2021),
            "2021" => Ok(StyleEdition::Edition2021),
            "2024" => Ok(StyleEdition::Edition2024),
            _ => Err(format!("Invalid rustfmt style_edition {s}")),
        }
    }
}

#[derive(Debug)]
pub struct FormatCodeError {
    error: String,
}

#[derive(Debug)]
pub enum CreateDiffError {
    /// Only the rustfmt binary compiled from the `main` branch failed
    MainRustfmtFailed(String),
    /// Only the rustfmt binary compiled form the `feature` branch failed
    FeatureRustfmtFailed(String),
    /// Both rustfmt binaries failed to format the input
    BothRustfmtFailed { src: String, feature: String },
    /// Some Error unrelated to formatting
    GeneralError,
}

#[derive(Debug)]
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
    /// Error formatting code
    FormatCodeError(FormatCodeError),
    /// Error generating diff
    CreateDiffError(CreateDiffError),
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

#[derive(Debug)]
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

pub struct Diff {
    src_format: String,
    feature_format: String,
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let patch = diffy::create_patch(self.src_format.as_str(), self.feature_format.as_str());
        write!(f, "{}", patch)
    }
}

impl Diff {
    pub fn is_empty(&self) -> bool {
        let patch = diffy::create_patch(self.src_format.as_str(), self.feature_format.as_str());
        patch.hunks().is_empty()
    }
}

pub struct CheckDiffRunners<F, S> {
    feature_runner: F,
    src_runner: S,
}

pub trait CodeFormatter {
    fn format_code_from_path<T: AsRef<str>>(
        &self,
        path: &Path,
        config: Option<&[T]>,
    ) -> Result<String, CheckDiffError> {
        let code = std::fs::read_to_string(path)?;
        self.format_code(&code, config)
    }

    fn format_code<T: AsRef<str>>(
        &self,
        code: &str,
        config: Option<&[T]>,
    ) -> Result<String, CheckDiffError>;
}

pub struct RustfmtRunner {
    ld_library_path: String,
    binary_path: PathBuf,
    edition: Edition,
    style_edition: StyleEdition,
}

impl<F, S> CheckDiffRunners<F, S> {
    pub fn new(feature_runner: F, src_runner: S) -> Self {
        Self {
            feature_runner,
            src_runner,
        }
    }
}

impl<F, S> CheckDiffRunners<F, S>
where
    F: CodeFormatter,
    S: CodeFormatter,
{
    /// Creates a diff generated by running the source and feature binaries on the same file path
    pub fn create_diff<T: AsRef<str>>(
        &self,
        path: &Path,
        additional_configs: Option<&[T]>,
    ) -> Result<Diff, CheckDiffError> {
        let src_format = self
            .src_runner
            .format_code_from_path(path, additional_configs);
        let feature_format = self
            .feature_runner
            .format_code_from_path(path, additional_configs);

        match (src_format, feature_format) {
            (Ok(s), Ok(f)) => Ok(Diff {
                src_format: s,
                feature_format: f,
            }),
            (Err(CheckDiffError::FormatCodeError(FormatCodeError { error })), Ok(_)) => {
                // src formatting failed.
                Err(CheckDiffError::CreateDiffError(
                    CreateDiffError::MainRustfmtFailed(error),
                ))
            }
            (Ok(_), Err(CheckDiffError::FormatCodeError(FormatCodeError { error }))) => {
                // feature formatting failed
                Err(CheckDiffError::CreateDiffError(
                    CreateDiffError::FeatureRustfmtFailed(error),
                ))
            }
            (
                Err(CheckDiffError::FormatCodeError(FormatCodeError { error: src_error })),
                Err(CheckDiffError::FormatCodeError(FormatCodeError {
                    error: feature_error,
                })),
            ) => {
                // Both src formatting and feature formatting failed
                Err(CheckDiffError::CreateDiffError(
                    CreateDiffError::BothRustfmtFailed {
                        src: src_error,
                        feature: feature_error,
                    },
                ))
            }
            _ => Err(CheckDiffError::CreateDiffError(
                CreateDiffError::GeneralError,
            )),
        }
    }
}

impl RustfmtRunner {
    fn get_binary_version(&self) -> Result<String, CheckDiffError> {
        let Ok(command) = Command::new(&self.binary_path)
            .env(dynamic_library_path_env_var_name(), &self.ld_library_path)
            .args(["--version"])
            .output()
        else {
            return Err(CheckDiffError::FailedBinaryVersioning(
                self.binary_path.clone(),
            ));
        };

        let binary_version = std::str::from_utf8(&command.stdout)?.trim();
        Ok(binary_version.to_string())
    }
}

// This is the same logic that cargo uses when setting thewse env vars
/// Returns the name of the environment variable used to search for dynamic libraries.
fn dynamic_library_path_env_var_name() -> &'static str {
    if cfg!(windows) {
        "PATH"
    } else if cfg!(target_os = "macos") {
        "DYLD_FALLBACK_LIBRARY_PATH"
    } else if cfg!(target_os = "aix") {
        "LIBPATH"
    } else {
        "LD_LIBRARY_PATH"
    }
}

impl CodeFormatter for RustfmtRunner {
    fn format_code_from_path<T: AsRef<str>>(
        &self,
        path: &Path,
        config: Option<&[T]>,
    ) -> Result<String, CheckDiffError> {
        let config = create_config_arg(config);
        let command = Command::new(&self.binary_path)
            .env(dynamic_library_path_env_var_name(), &self.ld_library_path)
            .args([
                "--edition",
                self.edition.as_str(),
                "--style-edition",
                self.style_edition.as_str(),
                "--unstable-features",
                "--skip-children",
                "--emit=stdout",
                config.as_str(),
            ])
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = command.wait_with_output()?;
        let errors = std::str::from_utf8(&output.stderr)?.to_string();
        match output.status.code() {
            Some(0) => Ok(std::str::from_utf8(&output.stdout)?.to_string()),
            Some(_) | None => {
                let formatted_code = std::str::from_utf8(&output.stdout)?.to_string();
                if !formatted_code.is_empty() {
                    Ok(formatted_code)
                } else {
                    debug!("{} {errors}", path.display());
                    Err(CheckDiffError::FormatCodeError(FormatCodeError {
                        error: errors,
                    }))
                }
            }
        }
    }

    //  Run rusfmt to see if a diff is produced. Runs on the code specified
    //
    // Parameters:
    // code: Code to run the binary on
    // config: Any additional configuration options to pass to rustfmt
    //
    fn format_code<T: AsRef<str>>(
        &self,
        code: &str,
        config: Option<&[T]>,
    ) -> Result<String, CheckDiffError> {
        let config = create_config_arg(config);
        let mut command = Command::new(&self.binary_path)
            .env(dynamic_library_path_env_var_name(), &self.ld_library_path)
            .args([
                "--edition",
                self.edition.as_str(),
                "--style-edition",
                self.style_edition.as_str(),
                "--unstable-features",
                "--skip-children",
                "--emit=stdout",
                config.as_str(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        command.stdin.as_mut().unwrap().write_all(code.as_bytes())?;
        let output = command.wait_with_output()?;
        let errors = std::str::from_utf8(&output.stderr)?.to_string();
        match output.status.code() {
            Some(0) => Ok(std::str::from_utf8(&output.stdout)?.to_string()),
            Some(_) | None => {
                let formatted_code = std::str::from_utf8(&output.stdout)?.to_string();
                if !formatted_code.is_empty() {
                    Ok(formatted_code)
                } else {
                    debug!("{errors}");
                    Err(CheckDiffError::FormatCodeError(FormatCodeError {
                        error: errors,
                    }))
                }
            }
        }
    }
}

/// Creates a configuration in the following form:
/// <config_name>=<config_val>, <config_name>=<config_val>, ...
fn create_config_arg<T: AsRef<str>>(config: Option<&[T]>) -> String {
    let config_arg: String = match config {
        Some(configs) => {
            let mut result = String::new();
            for arg in configs.iter() {
                result.push(',');
                result.push_str(arg.as_ref());
            }
            result
        }
        None => String::new(),
    };
    let config = format!(
        "--config=error_on_line_overflow=false,error_on_unformatted=false{}",
        config_arg.as_str()
    );
    config
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

    info!("Successfully cloned repository {url} to {}", dest.display());
    Ok(())
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
    Ok(())
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
    Ok(())
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
    Ok(())
}

pub fn change_directory_to_path(dest: &Path) -> io::Result<()> {
    let dest_path = Path::new(&dest);
    env::set_current_dir(dest_path)?;
    info!(
        "Setting current directory to: {}",
        env::current_dir().unwrap().display()
    );
    Ok(())
}

pub fn get_ld_library_path(dir: &Path) -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("rustc")
        .current_dir(dir)
        .args(["--print", "sysroot"])
        .output()
    else {
        return Err(CheckDiffError::FailedCommand("Error getting sysroot"));
    };
    let sysroot = std::str::from_utf8(&command.stdout)?.trim_end();
    let ld_lib_path = format!("{}/lib", sysroot);
    Ok(ld_lib_path)
}

pub fn get_cargo_version() -> Result<String, CheckDiffError> {
    let Ok(command) = Command::new("cargo").args(["--version"]).output() else {
        return Err(CheckDiffError::FailedCargoVersion(
            "Failed to obtain cargo version",
        ));
    };

    let cargo_version = std::str::from_utf8(&command.stdout)?.trim_end();
    Ok(cargo_version.to_string())
}

/// Obtains the ld_lib path and then builds rustfmt from source
/// If that operation succeeds, the source is then copied to the output path specified
pub fn build_rustfmt_from_src(
    binary_path: PathBuf,
    dir: &Path,
    edition: Edition,
    style_edition: StyleEdition,
) -> Result<RustfmtRunner, CheckDiffError> {
    //Because we're building standalone binaries we need to set `LD_LIBRARY_PATH` so each
    // binary can find it's runtime dependencies.
    // See https://github.com/rust-lang/rustfmt/issues/5675
    // This will prepend the `LD_LIBRARY_PATH` for the main rustfmt binary
    let ld_lib_path = get_ld_library_path(dir)?;

    info!("Building rustfmt from source");
    let Ok(_output) = Command::new("cargo")
        .args(["build", "-q", "--release", "--bin", "rustfmt"])
        .output()
    else {
        return Err(CheckDiffError::FailedSourceBuild(
            "Error building rustfmt from source",
        ));
    };

    std::fs::copy(dir.join("target/release/rustfmt"), &binary_path)?;

    Ok(RustfmtRunner {
        ld_library_path: ld_lib_path,
        binary_path,
        edition,
        style_edition,
    })
}

// Compiles and produces two rustfmt binaries.
// One for the current main branch, and another for the feature branch
// Parameters:
// dest: Directory where rustfmt will be cloned
pub fn compile_rustfmt(
    dest: &Path,
    remote_repo_url: String,
    feature_branch: String,
    edition: Edition,
    style_edition: StyleEdition,
    commit_hash: Option<String>,
) -> Result<CheckDiffRunners<RustfmtRunner, RustfmtRunner>, CheckDiffError> {
    const RUSTFMT_REPO: &str = "https://github.com/rust-lang/rustfmt.git";

    clone_git_repo(RUSTFMT_REPO, dest)?;
    change_directory_to_path(dest)?;
    git_remote_add(remote_repo_url.as_str())?;
    git_fetch(feature_branch.as_str())?;

    let cargo_version = get_cargo_version()?;
    info!("Compiling with {}", cargo_version);
    let src_runner =
        build_rustfmt_from_src(dest.join("src_rustfmt"), dest, edition, style_edition)?;
    let should_detach = commit_hash.is_some();
    git_switch(
        commit_hash.unwrap_or(feature_branch).as_str(),
        should_detach,
    )?;

    let feature_runner =
        build_rustfmt_from_src(dest.join("feature_rustfmt"), dest, edition, style_edition)?;
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

    Ok(CheckDiffRunners {
        src_runner,
        feature_runner,
    })
}

/// Searches for rust files in the particular path and returns an iterator to them.
pub fn search_for_rs_files(repo: &Path) -> impl Iterator<Item = PathBuf> {
    // TODO implement rustfmt ignore logic so that we skip ignored files.
    // There are a lot of ignored files in the r-l/rust repo so this should speed things up
    WalkDir::new(repo).into_iter().filter_map(|e| match e.ok() {
        Some(entry) => {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                return Some(entry.into_path());
            }
            None
        }
        None => None,
    })
}

/// Calculates the number of errors when running the compiled binary and the feature binary on the
/// repo specified with the specific configs.
pub fn check_diff<T: AsRef<str>>(
    config: Option<&[T]>,
    runners: &CheckDiffRunners<impl CodeFormatter, impl CodeFormatter>,
    repo: &Path,
    repo_url: &str,
) -> u8 {
    let mut errors: u8 = 0;
    let iter = search_for_rs_files(repo);
    for file in iter {
        let relative_path = file.strip_prefix(repo).unwrap_or(&file);
        let repo_name = get_repo_name(repo_url);
        trace!(
            "Formatting '{repo_name}' file {repo_name}/{}",
            relative_path.display()
        );
        match runners.create_diff(file.as_path(), config) {
            Ok(diff) => {
                if !diff.is_empty() {
                    error!(
                        "Diff found in '{0}' when formatting {0}/{1}\n{2}",
                        repo_name,
                        relative_path.display(),
                        diff,
                    );
                    errors = errors.saturating_add(1);
                } else {
                    trace!(
                        "No diff found in '{0}' when formatting {0}/{1}",
                        repo_name,
                        relative_path.display()
                    )
                }
            }
            Err(CheckDiffError::CreateDiffError(CreateDiffError::MainRustfmtFailed(_))) => {
                debug!(
                    "Couldn't create diff because `main` rustfmt failed to format {}/{}",
                    repo_name,
                    relative_path.display()
                );
                continue;
            }
            Err(CheckDiffError::CreateDiffError(CreateDiffError::FeatureRustfmtFailed(_))) => {
                debug!(
                    "Couldn't create diff because `feature` rustfmt failed to format {}/{}",
                    repo_name,
                    relative_path.display()
                );
                continue;
            }
            Err(CheckDiffError::CreateDiffError(CreateDiffError::BothRustfmtFailed { .. })) => {
                debug!(
                    "Couldn't create diff because both rustfmt binaries failed to format {}/{}",
                    repo_name,
                    relative_path.display()
                );
                continue;
            }
            Err(e) => {
                error!(
                    "Error creating diff for {}/{:?}: {:?}",
                    repo_name,
                    relative_path.display(),
                    e
                );
            }
        }
    }

    errors
}

/// parse out the repository name from a GitHub Repository name.
pub fn get_repo_name(git_url: &str) -> &str {
    let strip_git_prefix = &git_url[0..git_url.len() - 4];
    let (_, repo_name) = strip_git_prefix.rsplit_once('/').unwrap_or(("", git_url));
    repo_name
}
