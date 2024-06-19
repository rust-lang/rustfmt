use std::env;
use std::fs;
use std::path::Path;

pub const TEMP_DIR_PATH: &str = "./tmp/";
const HOME_PATH: &str = "./";

// removes the previous tmp folder if it exists
pub fn cleanup() {
    let _ = env::set_current_dir(HOME_PATH);
    println!(
        "Current directory: {}",
        env::current_dir().unwrap().display()
    );
    let path = Path::new(TEMP_DIR_PATH);
    println!("{}", path.exists());
    if path.exists() {
        let _ = fs::remove_dir_all(path);
    }
}
