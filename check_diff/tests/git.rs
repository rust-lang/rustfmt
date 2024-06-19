use check_diff::clone_git_repo;

use std::fs;
use std::path::Path;

use common::{cleanup, TEMP_DIR_PATH};

mod common;

#[test]
fn clone_repo_test() {
    cleanup();
    let sample_repo = "https://github.com/rust-lang/rustfmt.git";
    let dest_path = Path::new(TEMP_DIR_PATH);
    let result = clone_git_repo(sample_repo, dest_path);
    assert_eq!(result.is_ok(), true);
    let directory = fs::read_dir(dest_path);
    // check whether we can read this directory
    assert_eq!(directory.is_err(), false);
    // check that the directory is non-empty
    assert_eq!(directory.iter().next().is_none(), false);
    cleanup();
}
