use check_diff::clone_git_repo;

use std::fs;
use tempfile::Builder;

#[test]
fn clone_repo_test() {
    let dir = Builder::new().tempdir_in("");
    match dir {
        Ok(d) => {
            let sample_repo = "https://github.com/rust-lang/rustfmt.git";
            let dest_path = d.path();
            let result = clone_git_repo(sample_repo, dest_path);
            assert_eq!(result.is_ok(), true);
            let directory = fs::read_dir(dest_path);
            // check whether we can read this directory
            assert_eq!(directory.is_err(), false);
            // check that the directory is non-empty
            assert_eq!(directory.iter().next().is_none(), false);
            match d.close() {
                Ok(_) => {}
                Err(_) => {
                    assert_eq!(1, 2);
                }
            }
        }
        Err(_) => {
            assert_eq!(1, 2);
        }
    }
}
