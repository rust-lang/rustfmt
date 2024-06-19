use check_diff::change_directory_to_path;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use common::{cleanup, TEMP_DIR_PATH};

mod common;

#[test]
fn cd_test() {
    cleanup();
    let dest_path = Path::new(TEMP_DIR_PATH);
    let _ = fs::create_dir(dest_path);
    change_directory_to_path(dest_path);
    assert_eq!(env::current_dir().is_ok(), true);
    assert_eq!(
        env::current_dir().unwrap().file_name(),
        Some(OsStr::new("tmp"))
    );
    cleanup();
}
