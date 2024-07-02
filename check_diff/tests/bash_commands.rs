use check_diff::change_directory_to_path;
use std::env;
use std::ffi::OsStr;
use tempfile::Builder;
use tracing::error;

#[test]
fn cd_test() {
    // create directory in current directory
    let dir = Builder::new().tempdir_in("");

    match dir {
        Ok(d) => {
            let dest_path = d.path();
            let _ = change_directory_to_path(dest_path);
            assert_eq!(env::current_dir().is_ok(), true);
            assert_ne!(
                env::current_dir().unwrap().file_name(),
                Some(OsStr::new("check_diff"))
            );
            match d.close() {
                Ok(_) => {}
                Err(e) => {
                    error!("Error from closing: {}", e);
                    assert_eq!(1, 2);
                }
            }
        }
        Err(e) => {
            error!("Error from building: {}", e);
            assert_eq!(1, 2);
        }
    };
}
