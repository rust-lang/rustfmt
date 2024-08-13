use git::clone_git_repo;
use git::git_fetch;
use git::git_remote_add;
use std::env;
use std::io;
use std::io::Error;
use std::path::Path;
use std::process::Command;
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
pub fn compile_rustfmt(dest: &Path, inputs: CliInputs) -> io::Result<Command> {
    let Ok(_) = clone_git_repo(RUSTFMT_REPO, dest) else {
        return Err(Error::other("Error cloning repo while compiling rustfmt"));
    };

    let Ok(_) = git_remote_add(inputs.remote_repo_url.as_str()) else {
        return Err(Error::other(format!(
            "Error adding remote from {} while compiling rustfmt",
            inputs.remote_repo_url
        )));
    };

    let Ok(_) = git_fetch(inputs.feature_branch.as_str()) else {
        return Err(Error::other(format!(
            "Error fetching from {} while compiling rustfmt",
            inputs.feature_branch
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
