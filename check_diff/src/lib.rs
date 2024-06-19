use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub struct GitErrors {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

/// Clone a git repository
///
/// Parameters:
/// url: git clone url
/// dest: directory where the repo should be cloned
pub fn clone_git_repo(url: &str, dest: &Path) -> Result<(), GitErrors> {
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
        .output()
        .expect("failed to clone repository");
    // if the git command does not return successfully,
    // any command on the repo will fail. So fail fast.
    if !git_cmd.status.success() {
        io::stdout().write_all(&git_cmd.stdout).unwrap();
        io::stderr().write_all(&git_cmd.stderr).unwrap();
        let errors = GitErrors {
            stderr: git_cmd.stderr,
            stdout: git_cmd.stdout,
        };
        return Err(errors);
    }
    println!("Successfully clone repository.");
    return Ok(());
}

pub fn change_directory_to_path(dest: &Path) {
    let dest_path = Path::new(&dest);
    let _ = env::set_current_dir(&dest_path).is_ok();
    println!(
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
}
