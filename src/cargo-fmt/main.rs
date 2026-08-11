// Inspired by Paul Woolcock's cargo-fmt (https://github.com/pwoolcoc/cargo-fmt/).

#![deny(warnings)]
#![allow(clippy::match_like_matches_macro)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use cargo_metadata::Edition;
use clap::{CommandFactory, Parser};
use tempfile::NamedTempFile;

#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

#[path = "test/mod.rs"]
#[cfg(test)]
mod cargo_fmt_tests;

const fn is_nightly() -> bool {
    match option_env!("CFG_RELEASE_CHANNEL") {
        None => true,
        Some(c) => matches!(c.as_bytes(), b"nightly" | b"dev"),
    }
}

const MESSAGE_FORMATS: &str = if is_nightly() {
    "short|json|human"
} else {
    "short|human"
};

#[derive(Parser)]
#[command(
    disable_version_flag = true,
    bin_name = "cargo fmt",
    about = "This utility formats all bin and lib files of \
             the current crate using rustfmt."
)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Opts {
    /// No output printed to stdout
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Use verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Print rustfmt version and exit
    #[arg(long = "version")]
    version: bool,

    /// Specify package to format
    #[arg(
        short = 'p',
        long = "package",
        value_name = "package",
        num_args = 1..
    )]
    packages: Vec<String>,

    /// Specify path to Cargo.toml
    #[arg(long = "manifest-path", value_name = "manifest-path")]
    manifest_path: Option<String>,

    #[arg(
        long = "message-format",
        value_name = "message-format",
        help = format!("Specify message-format: {MESSAGE_FORMATS}")
    )]
    message_format: Option<String>,

    /// Options passed to rustfmt
    // 'raw = true' to make `--` explicit.
    #[arg(id = "rustfmt_options", raw = true)]
    rustfmt_options: Vec<String>,

    /// Format all packages, and also their local path-based dependencies
    #[arg(long = "all")]
    format_all: bool,

    /// Run rustfmt in check mode
    #[arg(long = "check")]
    check: bool,
}

fn main() {
    let exit_status = execute();
    std::io::stdout().flush().unwrap();
    std::process::exit(exit_status);
}

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;

fn execute() -> i32 {
    // Drop extra `fmt` argument provided by `cargo`.
    let mut found_fmt = false;
    let args = env::args().filter(|x| {
        if found_fmt {
            true
        } else {
            found_fmt = x == "fmt";
            x != "fmt"
        }
    });

    let opts = Opts::parse_from(args);

    let verbosity = match (opts.verbose, opts.quiet) {
        (false, false) => Verbosity::Normal,
        (false, true) => Verbosity::Quiet,
        (true, false) => Verbosity::Verbose,
        (true, true) => {
            print_usage_to_stderr("quiet mode and verbose mode are not compatible");
            return FAILURE;
        }
    };

    if opts.version {
        return handle_command_status(get_rustfmt_info(&[String::from("--version")]));
    }
    if opts.rustfmt_options.iter().any(|s| {
        ["--print-config", "-h", "--help", "-V", "--version"].contains(&s.as_str())
            || s.starts_with("--help=")
            || s.starts_with("--print-config=")
    }) {
        return handle_command_status(get_rustfmt_info(&opts.rustfmt_options));
    }

    let strategy = CargoFmtStrategy::from_opts(&opts);
    let mut rustfmt_args = opts.rustfmt_options;
    if opts.check {
        let check_flag = "--check";
        if !rustfmt_args.iter().any(|o| o == check_flag) {
            rustfmt_args.push(check_flag.to_owned());
        }
    }
    if let Some(message_format) = opts.message_format {
        if let Err(msg) = convert_message_format_to_rustfmt_args(&message_format, &mut rustfmt_args)
        {
            print_usage_to_stderr(&msg);
            return FAILURE;
        }
    }

    if let Some(specified_manifest_path) = opts.manifest_path {
        if !specified_manifest_path.ends_with("Cargo.toml") {
            print_usage_to_stderr("the manifest-path must be a path to a Cargo.toml file");
            return FAILURE;
        }
        let manifest_path = PathBuf::from(specified_manifest_path);
        handle_command_status(format_crate(
            verbosity,
            &strategy,
            rustfmt_args,
            Some(&manifest_path),
        ))
    } else {
        handle_command_status(format_crate(verbosity, &strategy, rustfmt_args, None))
    }
}

fn rustfmt_path() -> PathBuf {
    match env::var_os("RUSTFMT") {
        Some(rustfmt) => PathBuf::from(rustfmt),
        None => env::current_exe()
            .expect("current executable path invalid")
            .with_file_name("rustfmt"),
    }
}

fn rustfmt_command() -> Command {
    Command::new(rustfmt_path())
}

fn convert_message_format_to_rustfmt_args(
    message_format: &str,
    rustfmt_args: &mut Vec<String>,
) -> Result<(), String> {
    let mut contains_emit_mode = false;
    let mut contains_check = false;
    let mut contains_list_files = false;
    for arg in rustfmt_args.iter() {
        if arg.starts_with("--emit") {
            contains_emit_mode = true;
        }
        if arg == "--check" {
            contains_check = true;
        }
        if arg == "-l" || arg == "--files-with-diff" {
            contains_list_files = true;
        }
    }
    match message_format {
        "short" => {
            if !contains_list_files {
                rustfmt_args.push(String::from("-l"));
            }
            Ok(())
        }
        "json" => {
            if !is_nightly() {
                return Err(String::from(
                    "--message-format json is only supported in nightly builds",
                ));
            }
            if contains_emit_mode {
                return Err(String::from(
                    "cannot include --emit arg when --message-format is set to json",
                ));
            }
            if contains_check {
                return Err(String::from(
                    "cannot include --check arg when --message-format is set to json",
                ));
            }
            rustfmt_args.push(String::from("--emit"));
            rustfmt_args.push(String::from("json"));
            Ok(())
        }
        "human" => Ok(()),
        _ => Err(format!(
            "invalid --message-format value: {message_format}. Allowed values are: \
                {MESSAGE_FORMATS}"
        )),
    }
}

fn print_usage_to_stderr(reason: &str) {
    eprintln!("{reason}");
    let app = Opts::command();
    let help = app.after_help("").render_help();
    eprintln!("{help}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    Normal,
    Quiet,
}

fn handle_command_status(status: Result<i32, io::Error>) -> i32 {
    match status {
        Err(e) => {
            print_usage_to_stderr(&e.to_string());
            FAILURE
        }
        Ok(status) => status,
    }
}

fn get_rustfmt_info(args: &[String]) -> Result<i32, io::Error> {
    let mut command = rustfmt_command()
        .stdout(std::process::Stdio::inherit())
        .args(args)
        .spawn()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => io::Error::new(
                io::ErrorKind::Other,
                "Could not run rustfmt, please make sure it is in your PATH.",
            ),
            _ => e,
        })?;
    let result = command.wait()?;
    if result.success() {
        Ok(SUCCESS)
    } else {
        Ok(result.code().unwrap_or(SUCCESS))
    }
}

fn format_crate(
    verbosity: Verbosity,
    strategy: &CargoFmtStrategy,
    rustfmt_args: Vec<String>,
    manifest_path: Option<&Path>,
) -> Result<i32, io::Error> {
    let targets = get_targets(strategy, manifest_path)?;

    // Currently only bin and lib files get formatted.
    run_rustfmt(&targets, &rustfmt_args, verbosity)
}

/// Target uses a `path` field for equality and hashing.
#[derive(Debug)]
pub struct Target {
    /// A path to the main source file of the target.
    path: PathBuf,
    /// A kind of target (e.g., lib, bin, example, ...).
    kind: String,
    /// Rust edition for this target.
    edition: Edition,
}

impl Target {
    pub fn from_target(target: &cargo_metadata::Target) -> Self {
        let path = PathBuf::from(&target.src_path);
        let canonicalized = fs::canonicalize(&path).unwrap_or(path);

        Target {
            path: canonicalized,
            kind: target.kind[0].to_string(),
            edition: target.edition,
        }
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Target) -> bool {
        self.path == other.path
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Target) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Target) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl Eq for Target {}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CargoFmtStrategy {
    /// Format every packages and dependencies.
    All,
    /// Format packages that are specified by the command line argument.
    Some(Vec<String>),
    /// Format the root packages only.
    Root,
}

impl CargoFmtStrategy {
    pub fn from_opts(opts: &Opts) -> CargoFmtStrategy {
        match (opts.format_all, opts.packages.is_empty()) {
            (false, true) => CargoFmtStrategy::Root,
            (true, _) => CargoFmtStrategy::All,
            (false, false) => CargoFmtStrategy::Some(opts.packages.clone()),
        }
    }
}

/// Based on the specified `CargoFmtStrategy`, returns a set of main source files.
fn get_targets(
    strategy: &CargoFmtStrategy,
    manifest_path: Option<&Path>,
) -> Result<BTreeSet<Target>, io::Error> {
    let mut targets = BTreeSet::new();

    match *strategy {
        CargoFmtStrategy::Root => get_targets_root_only(manifest_path, &mut targets)?,
        CargoFmtStrategy::All => {
            get_targets_recursive(manifest_path, &mut targets, &mut BTreeSet::new())?
        }
        CargoFmtStrategy::Some(ref hitlist) => {
            get_targets_with_hitlist(manifest_path, hitlist, &mut targets)?
        }
    }

    if targets.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to find targets".to_owned(),
        ))
    } else {
        Ok(targets)
    }
}

fn get_targets_root_only(
    manifest_path: Option<&Path>,
    targets: &mut BTreeSet<Target>,
) -> Result<(), io::Error> {
    let metadata = get_cargo_metadata(manifest_path)?;
    let workspace_root_path = PathBuf::from(&metadata.workspace_root).canonicalize()?;
    let (in_workspace_root, current_dir_manifest) = if let Some(target_manifest) = manifest_path {
        (
            workspace_root_path == target_manifest,
            target_manifest.canonicalize()?,
        )
    } else {
        let current_dir = env::current_dir()?.canonicalize()?;
        (
            workspace_root_path == current_dir,
            current_dir.join("Cargo.toml"),
        )
    };

    let package_targets = match metadata.packages.len() {
        1 => metadata.packages.into_iter().next().unwrap().targets,
        _ => metadata
            .packages
            .into_iter()
            .filter(|p| {
                in_workspace_root
                    || PathBuf::from(&p.manifest_path)
                        .canonicalize()
                        .unwrap_or_default()
                        == current_dir_manifest
            })
            .flat_map(|p| p.targets)
            .collect(),
    };

    for target in package_targets {
        targets.insert(Target::from_target(&target));
    }

    Ok(())
}

fn get_targets_recursive(
    manifest_path: Option<&Path>,
    targets: &mut BTreeSet<Target>,
    visited: &mut BTreeSet<String>,
) -> Result<(), io::Error> {
    let metadata = get_cargo_metadata(manifest_path)?;
    for package in &metadata.packages {
        add_targets(&package.targets, targets);

        // Look for local dependencies using information available since cargo v1.51
        // It's theoretically possible someone could use a newer version of rustfmt with
        // a much older version of `cargo`, but we don't try to explicitly support that scenario.
        // If someone reports an issue with path-based deps not being formatted, be sure to
        // confirm their version of `cargo` (not `cargo-fmt`) is >= v1.51
        // https://github.com/rust-lang/cargo/pull/8994
        for dependency in &package.dependencies {
            if dependency.path.is_none() || visited.contains(&dependency.name) {
                continue;
            }

            let manifest_path = PathBuf::from(dependency.path.as_ref().unwrap()).join("Cargo.toml");
            if manifest_path.exists()
                && !metadata
                    .packages
                    .iter()
                    .any(|p| p.manifest_path.eq(&manifest_path))
            {
                visited.insert(dependency.name.to_owned());
                get_targets_recursive(Some(&manifest_path), targets, visited)?;
            }
        }
    }

    Ok(())
}

fn get_targets_with_hitlist(
    manifest_path: Option<&Path>,
    hitlist: &[String],
    targets: &mut BTreeSet<Target>,
) -> Result<(), io::Error> {
    let metadata = get_cargo_metadata(manifest_path)?;
    let mut workspace_hitlist: BTreeSet<&str> =
        BTreeSet::from_iter(hitlist.into_iter().map(|s| s.as_str()));

    for package in metadata.packages {
        if workspace_hitlist.remove(package.name.as_ref()) {
            for target in package.targets {
                targets.insert(Target::from_target(&target));
            }
        }
    }

    if workspace_hitlist.is_empty() {
        Ok(())
    } else {
        let package = workspace_hitlist.iter().next().unwrap();
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("package `{package}` is not a member of the workspace"),
        ))
    }
}

fn add_targets(target_paths: &[cargo_metadata::Target], targets: &mut BTreeSet<Target>) {
    for target in target_paths {
        targets.insert(Target::from_target(target));
    }
}

fn expand_args_file_args(args: &[OsString]) -> Result<Vec<OsString>, io::Error> {
    let mut options_enabled = true;
    expand_args_file_args_inner(args, 0, &mut options_enabled)
}

fn expand_args_file_args_inner(
    args: &[OsString],
    depth: usize,
    options_enabled: &mut bool,
) -> Result<Vec<OsString>, io::Error> {
    const MAX_ARGS_FILE_DEPTH: usize = 16;
    if depth > MAX_ARGS_FILE_DEPTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("argument files cannot be nested more than {MAX_ARGS_FILE_DEPTH} levels"),
        ));
    }

    let mut expanded = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if !*options_enabled {
            expanded.push(arg.clone());
            expanded.extend(args.cloned());
            break;
        }
        let arg = arg.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "rustfmt argument-file arguments must be valid UTF-8",
            )
        })?;
        if arg == "--" {
            *options_enabled = false;
            expanded.push(OsString::from(arg));
            expanded.extend(args.cloned());
            break;
        }
        let path = if arg == "--args-file" {
            Some(
                args.next()
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "`--args-file` requires a path")
                    })?
                    .to_str()
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "argument-file paths must be valid UTF-8",
                        )
                    })?
                    .to_owned(),
            )
        } else {
            arg.strip_prefix("--args-file=").map(str::to_owned)
        };

        if let Some(path) = path {
            if path.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "`--args-file` requires a path",
                ));
            }
            let contents = fs::read_to_string(&path).map_err(|e| {
                io::Error::new(
                    e.kind(),
                    format!("failed to load argument file `{path}`: {e}"),
                )
            })?;
            let nested = contents.lines().map(OsString::from).collect::<Vec<_>>();
            expanded.extend(expand_args_file_args_inner(
                &nested,
                depth + 1,
                options_enabled,
            )?);
            if !*options_enabled {
                expanded.extend(args.cloned());
                break;
            }
        } else {
            expanded.push(OsString::from(arg));
        }
    }
    Ok(expanded)
}

fn write_args_file(args: &[OsString]) -> Result<NamedTempFile, io::Error> {
    let mut args_file = NamedTempFile::new()?;
    for arg in expand_args_file_args(args)? {
        let arg = arg.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "rustfmt argument-file arguments must be valid UTF-8",
            )
        })?;
        if arg.contains('\n') || arg.contains('\r') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "rustfmt argument-file arguments cannot contain newlines",
            ));
        }
        writeln!(args_file, "{arg}")?;
    }
    args_file.flush()?;
    Ok(args_file)
}

#[cfg(windows)]
fn command_line_arg_len(arg: &OsStr) -> usize {
    let arg = arg.encode_wide().collect::<Vec<_>>();
    let quoted = arg.is_empty() || arg.iter().any(|&c| c == b' ' as u16 || c == b'\t' as u16);
    let mut len = arg.len().saturating_add(usize::from(quoted) * 2);
    let mut backslashes = 0usize;

    for &c in &arg {
        if c == b'\\' as u16 {
            backslashes += 1;
        } else {
            if c == b'"' as u16 {
                len = len.saturating_add(backslashes).saturating_add(1);
            }
            backslashes = 0;
        }
    }
    if quoted {
        len = len.saturating_add(backslashes);
    }

    // Account for the separator before this argument.
    len.saturating_add(1)
}

#[cfg(windows)]
fn command_line_program_len(program: &Path) -> usize {
    // Rust always surrounds argv[0] with quotes on Windows.
    program.as_os_str().encode_wide().count().saturating_add(2)
}

#[cfg(windows)]
fn should_use_args_file(program: &Path, args: &[OsString]) -> bool {
    const WINDOWS_COMMAND_LINE_LIMIT: usize = 32_767;

    let command_line_len = command_line_program_len(program).saturating_add(
        args.iter()
            .map(|arg| command_line_arg_len(arg))
            .sum::<usize>(),
    );
    command_line_len >= WINDOWS_COMMAND_LINE_LIMIT
}

#[cfg(not(windows))]
fn should_use_args_file(_program: &Path, _args: &[OsString]) -> bool {
    false
}

fn run_rustfmt(
    targets: &BTreeSet<Target>,
    fmt_args: &[String],
    verbosity: Verbosity,
) -> Result<i32, io::Error> {
    let by_edition = targets
        .iter()
        .inspect(|t| {
            if verbosity == Verbosity::Verbose {
                println!("[{} ({})] {:?}", t.kind, t.edition, t.path)
            }
        })
        .fold(BTreeMap::new(), |mut h, t| {
            h.entry(&t.edition).or_insert_with(Vec::new).push(&t.path);
            h
        });

    let mut status = vec![];
    for (edition, files) in by_edition {
        let stdout = if verbosity == Verbosity::Quiet {
            std::process::Stdio::null()
        } else {
            std::process::Stdio::inherit()
        };

        if verbosity == Verbosity::Verbose {
            print!("rustfmt");
            print!(" --edition {edition}");
            fmt_args.iter().for_each(|f| print!(" {}", f));
            files.iter().for_each(|f| print!(" {}", f.display()));
            println!();
        }

        let rustfmt = rustfmt_path();
        let mut args = files
            .iter()
            .map(|file| file.as_os_str().to_owned())
            .collect::<Vec<_>>();
        args.extend([
            OsString::from("--edition"),
            OsString::from(edition.as_str()),
        ]);
        args.extend(fmt_args.iter().map(OsString::from));

        let args_file = should_use_args_file(&rustfmt, &args)
            .then(|| write_args_file(&args))
            .transpose()?;

        let mut command = Command::new(rustfmt);
        command.stdout(stdout);
        if let Some(args_file) = args_file.as_ref() {
            command.arg("--args-file").arg(args_file.path());
        } else {
            command.args(&args);
        }

        let mut command = command.spawn().map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => io::Error::new(
                io::ErrorKind::Other,
                "Could not run rustfmt, please make sure it is in your PATH.",
            ),
            _ => e,
        })?;

        status.push(command.wait()?);
    }

    Ok(status
        .iter()
        .filter_map(|s| if s.success() { None } else { s.code() })
        .next()
        .unwrap_or(SUCCESS))
}

fn get_cargo_metadata(manifest_path: Option<&Path>) -> Result<cargo_metadata::Metadata, io::Error> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    if let Some(manifest_path) = manifest_path {
        cmd.manifest_path(manifest_path);
    }
    cmd.other_options(vec![String::from("--offline")]);

    match cmd.exec() {
        Ok(metadata) => Ok(metadata),
        Err(_) => {
            cmd.other_options(vec![]);
            match cmd.exec() {
                Ok(metadata) => Ok(metadata),
                Err(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
            }
        }
    }
}
