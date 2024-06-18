use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Clone a git repository
///
/// Parameters:
/// url: git clone url
/// dest: directory where the repo should be cloned
pub fn clone_git_repo(url: &str, dest: &Path) {
    env::set_var("GIT_TERMINAL_PROMPT", "0");
    let git_cmd = Command::new("git")
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
    // cd command will not work as well. So fail fast.
    if !git_cmd.status.success() {
        io::stdout().write_all(&git_cmd.stdout).unwrap();
        io::stderr().write_all(&git_cmd.stderr).unwrap();
        return;
    }
    println!("Successfully clone repository. Entering repository");
}

pub fn change_directory_to_path(dest: &Path) {
    let dest_path = Path::new(&dest);
    let _ = env::set_current_dir(&dest_path).is_ok();
    println!(
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
}
