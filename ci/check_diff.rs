#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! tempfile = "3.2"
//! ```

use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

fn print_usage() {
    println!("usage check_diff REMOTE_REPO FEATURE_BRANCH [COMMIT_HASH] [OPTIONAL_RUSTFMT_CONFIGS]");
}

fn clone_repo(repo_url: &str, directory: &str) -> io::Result<()> {
    let _output = Command::new("git")
        .arg("clone")
        .arg("--quiet")
        .arg(repo_url)
        .arg("--depth")
        .arg("1")
        .arg(directory)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()?;
    Ok(())
}

fn init_submodules(dirs: &[&str]) -> io::Result<()> {
    let _output = Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .args(dirs)
        .output()?;
    Ok(())
}

fn create_diff(rustfmt_bin: &str, output_file: &str, optional_rustfmt_configs: &str) -> io::Result<()> {
    let config = if optional_rustfmt_configs.is_empty() {
        String::from("--config=error_on_line_overflow=false,error_on_unformatted=false")
    } else {
        format!(
            "--config=error_on_line_overflow=false,error_on_unformatted=false,{}",
            optional_rustfmt_configs
        )
    };

    let files = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some("rs".as_ref()))
        .filter_map(|entry| entry.path().to_str().map(String::from));

    let mut output_file = File::create(output_file)?;

    for file in files {
        let output = Command::new(rustfmt_bin)
            .arg("--unstable-features")
            .arg("--skip-children")
            .arg("--check")
            .arg("--color=always")
            .arg(&config) // Pass config by value
            .arg(&file)
            .stderr(std::process::Stdio::null())
            .output()?;
        output_file.write_all(&output.stdout)?;
    }
    Ok(())
}

fn check_diff(repo_name: &str, rusfmt_bin: &str, feature_bin: &str, optional_rustfmt_configs: &str) -> io::Result<bool> {
    println!("running rustfmt (master) on {}", repo_name);
    create_diff(rusfmt_bin, "rustfmt_diff.txt", optional_rustfmt_configs)?;

    println!("running rustfmt (feature) on {}", repo_name);
    create_diff(feature_bin, "feature_diff.txt", optional_rustfmt_configs)?;

    println!("checking diff");

    let diff_output = Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .arg("--color=never")
        .arg("--unified=0")
        .arg("--no-index")
        .arg("rustfmt_diff.txt")
        .arg("feature_diff.txt")
        .output()?;

    let diff = String::from_utf8_lossy(&diff_output.stdout);
    if diff.is_empty() {
        println!("no diff detected between rustfmt and the feature branch");
        Ok(true)
    } else {
        println!("{}", diff);
        Ok(false)
    }
}

fn compile_rustfmt(repo_dir: &str, remote_repo: &str, feature_branch: &str, optional_commit_hash: Option<&str>) -> io::Result<(String, String)> {
    let rustfmt_repo = "https://github.com/rust-lang/rustfmt.git";
    clone_repo(rustfmt_repo, repo_dir)?;

    let _output = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("feature")
        .arg(remote_repo)
        .current_dir(repo_dir)
        .output()?;
    
    let _output = Command::new("git")
        .arg("fetch")
        .arg("feature")
        .arg(feature_branch)
        .current_dir(repo_dir)
        .output()?;

    let cargo_version_output = Command::new("cargo")
        .arg("--version")
        .output()?;
    let cargo_version = String::from_utf8_lossy(&cargo_version_output.stdout);
    println!("\ncompiling with {}\n", cargo_version);

    let sysroot_output = Command::new("rustc")
        .arg("--print")
        .arg("sysroot")
        .output()?;
    let sysroot = String::from_utf8_lossy(&sysroot_output.stdout).trim().to_owned();

    // Build rustfmt for master
    let _output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("rustfmt")
        .current_dir(repo_dir)
        .output()?;
    fs::copy(format!("{}/target/release/rustfmt", repo_dir), format!("{}/rustfmt", repo_dir))?;

    // Switch to feature branch or commit hash
    if let Some(commit_hash) = optional_commit_hash {
        let _output = Command::new("git")
            .arg("switch")
            .arg(commit_hash)
            .arg("--detach")
            .current_dir(repo_dir)
            .output()?;
    } else {
        let _output = Command::new("git")
            .arg("switch")
            .arg(feature_branch)
            .current_dir(repo_dir)
            .output()?;
    }

    // Build rustfmt for feature branch
    let _output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("rustfmt")
        .current_dir(repo_dir)
        .output()?;
    fs::copy(format!("{}/target/release/rustfmt", repo_dir), format!("{}/feature_rustfmt", repo_dir))?;

    let rusfmt_bin = format!("{}/rustfmt", repo_dir);
    let feature_bin = format!("{}/feature_rustfmt", repo_dir);

    println!("\nRuntime dependencies for rustfmt -- LD_LIBRARY_PATH: {}\n", sysroot);

    Ok((rusfmt_bin, feature_bin))
}

fn check_repo(repo_url: &str, repo_name: &str, submodules: Option<&[&str]>, rusfmt_bin: &str, feature_bin: &str, optional_rustfmt_configs: &str) -> io::Result<bool> {
    let tmp_dir = tempfile::tempdir()?;
    let tmp_dir_path = tmp_dir.path().to_str().unwrap();

    clone_repo(repo_url, tmp_dir_path)?;

    if let Some(submodules) = submodules {
        init_submodules(submodules)?;
    }

    let result = check_diff(repo_name, rusfmt_bin, feature_bin, optional_rustfmt_configs)?;

    println!("removing tmp_dir {}", tmp_dir_path);
    drop(tmp_dir);

    Ok(result)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        print_usage();
        std::process::exit(1);
    }

    let remote_repo = &args[1];
    let feature_branch = &args[2];
    let optional_commit_hash = args.get(3).cloned();
    let optional_rustfmt_configs = args.get(4).cloned().unwrap_or_default();

    let (rusfmt_bin, feature_bin) = compile_rustfmt("rustfmt_tmp", remote_repo, feature_branch, optional_commit_hash.as_deref())?;

    let repos = [
        ("https://github.com/rust-lang/rust.git", "rust-lang-rust", None),
        ("https://github.com/rust-lang/cargo.git", "cargo", None),
        ("https://github.com/rust-lang/miri.git", "miri", None),
        ("https://github.com/rust-lang/rust-analyzer.git", "rust-analyzer", None),
        ("https://github.com/bitflags/bitflags.git", "bitflags", None),
        ("https://github.com/rust-lang/log.git", "log", None),
        ("https://github.com/rust-lang/mdBook.git", "mdBook", None),
        ("https://github.com/rust-lang/packed_simd.git", "packed_simd", None),
        ("https://github.com/rust-lang/rust-semverver.git", "rust-semverver", None),
        ("https://github.com/Stebalien/tempfile.git", "tempfile", None),
        ("https://github.com/rust-lang/futures-rs.git", "futures-rs", None),
        ("https://github.com/dtolnay/anyhow.git", "anyhow", None),
        ("https://github.com/dtolnay/thiserror.git", "thiserror", None),
        ("https://github.com/dtolnay/syn.git", "syn", None),
        ("https://github.com/serde-rs/serde.git", "serde", None),
        ("https://github.com/rust-lang/rustlings.git", "rustlings", None),
        ("https://github.com/rust-lang/rustup.git", "rustup", None),
        ("https://github.com/SergioBenitez/Rocket.git", "Rocket", None),
        ("https://github.com/rustls/rustls.git", "rustls", None),
        ("https://github.com/rust-lang/rust-bindgen.git", "rust-bindgen", None),
        ("https://github.com/hyperium/hyper.git", "hyper", None),
        ("https://github.com/actix/actix.git", "actix", None),
        ("https://github.com/denoland/deno.git", "denoland_deno", None),
    ];

    let mut statuses = Vec::new();

    for &(repo_url, repo_name, submodules) in &repos {
        let status = match check_repo(repo_url, repo_name, submodules, &rusfmt_bin, &feature_bin, &optional_rustfmt_configs) {
            Ok(true) => {
                println!("no diff found 😊");
                0
            }
            Ok(false) => {
                println!("formatting diff found 💔");
                1
            }
            Err(_) => {
                println!("Error checking diff for {}", repo_name);
                1
            }
        };
        statuses.push(status);
    }

    if statuses.iter().any(|&status| status == 1) {
        std::process::exit(1);
    }

    Ok(())
}
