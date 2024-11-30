use check_diff::{check_diff, compile_rustfmt, search_for_rs_files, CheckDiffError};
use std::fs::File;
use tempfile::Builder;

#[test]
fn search_for_files_correctly_non_nested() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Builder::new().tempdir_in("").unwrap();
    let file_path = dir.path().join("test.rs");
    let _tmp_file = File::create(file_path)?;

    let iter = search_for_rs_files(dir.path());

    let mut count = 0;
    for _ in iter {
        count += 1;
    }

    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn search_for_files_correctly_nested() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Builder::new().tempdir_in("").unwrap();
    let file_path = dir.path().join("test.rs");
    let _tmp_file = File::create(file_path)?;

    let nested_dir = Builder::new().tempdir_in(dir.path()).unwrap();
    let nested_file_path = nested_dir.path().join("nested.rs");
    let _ = File::create(nested_file_path)?;

    let iter = search_for_rs_files(dir.path());

    let mut count = 0;
    for _ in iter {
        count += 1;
    }

    assert_eq!(count, 2);

    Ok(())
}

#[test]
fn check_diff_test() -> Result<(), CheckDiffError> {
    let tmp_dir = Builder::new().tempdir_in("").unwrap();
    let runners = compile_rustfmt(
        tmp_dir.path(),
        "https://github.com/rust-lang/rustfmt".to_string(),
        "rustfmt-1.4.32".to_string(),
        None,
    )?;

    let dir = Builder::new().tempdir_in("").unwrap();
    let file_path = dir.path().join("test.rs");
    let _tmp_file = File::create(file_path)?;

    let errors = check_diff(None, runners, dir.path());
    assert_eq!(errors, 0);
    Ok(())
}

#[test]
fn format_simple_code() -> Result<(), CheckDiffError> {
    let tmp_dir = Builder::new().tempdir_in("").unwrap();
    let runners = compile_rustfmt(
        tmp_dir.path(),
        "https://github.com/rust-lang/rustfmt".to_string(),
        "rustfmt-1.4.32".to_string(),
        None,
    )?;

    let output = runners
        .src_runner
        .format_code("fn main()              {}", &None)?;
    assert_eq!(output, "fn main() {}\n".to_string());

    Ok(())
}
