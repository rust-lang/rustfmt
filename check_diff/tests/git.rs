use check_diff::{change_directory_to_path, clone_git_repo};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

const TEMP_DIR_PATH: &str = "./tmp/";

// removes the previous tmp folder if it exists
#[macro_export]
macro_rules! cleanup {
    () => {
        if (Path::new(TEMP_DIR_PATH).exists()) {
            let _ = fs::remove_dir_all(TEMP_DIR_PATH);
        }
    };
}

#[test]
fn clone_repo_test() {
    let sample_repo = "https://github.com/rust-lang/rustfmt.git";
    let dest_path = Path::new(TEMP_DIR_PATH);
    clone_git_repo(sample_repo, dest_path);
    let directory = fs::read_dir(TEMP_DIR_PATH);
    // check whether we can read this directory
    assert_eq!(directory.is_err(), false);
    // check that the directory is non-empty
    assert_eq!(directory.iter().next().is_none(), false);
    cleanup!();
}

#[test]
fn cd_test() {
    let dest_path = Path::new(TEMP_DIR_PATH);
    let _ = fs::create_dir(dest_path);
    change_directory_to_path(dest_path);
    assert_eq!(env::current_dir().is_ok(), true);
    assert_eq!(
        env::current_dir().unwrap().file_name(),
        Some(OsStr::new("tmp"))
    );
    cleanup!();
}
