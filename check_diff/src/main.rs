use std::io::Error;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use check_diff::{
    Edition, StyleEdition, check_diff, clone_git_repo, compile_rustfmt, get_repo_name,
};
use clap::Parser;
use tempfile::tempdir;
use tracing::{error, info, warn};

/// A curated set of `rust-lang/*` and popular ecosystem repositories to compare `rustfmt`s against.
const REPOS: &[&str] = &[
    // `rust-lang/*` repositories.
    "https://github.com/rust-lang/cargo.git",
    "https://github.com/rust-lang/futures-rs.git",
    "https://github.com/rust-lang/log.git",
    "https://github.com/rust-lang/mdBook.git",
    "https://github.com/rust-lang/miri.git",
    "https://github.com/rust-lang/packed_simd.git",
    "https://github.com/rust-lang/rust-analyzer.git",
    "https://github.com/rust-lang/rust-bindgen.git",
    "https://github.com/rust-lang/rust-semverver.git",
    "https://github.com/rust-lang/rust.git",
    "https://github.com/rust-lang/rustlings.git",
    "https://github.com/rust-lang/rustup.git",
    // Ecosystem repositories
    "https://github.com/actix/actix.git",
    "https://github.com/bitflags/bitflags.git",
    "https://github.com/denoland/deno.git",
    "https://github.com/dtolnay/anyhow.git",
    "https://github.com/dtolnay/syn.git",
    "https://github.com/dtolnay/thiserror.git",
    "https://github.com/hyperium/hyper.git",
    "https://github.com/rustls/rustls.git",
    "https://github.com/serde-rs/serde.git",
    "https://github.com/SergioBenitez/Rocket.git",
    "https://github.com/Stebalien/tempfile.git",
];

/// Inputs for the check_diff script
#[derive(Parser)]
struct CliInputs {
    /// Git url of a rustfmt fork to compare against the latest main rustfmt
    remote_repo_url: String,
    /// Name of the feature branch on the forked repo
    feature_branch: String,
    /// Rust language `edition` used to parse code. Possible values {2015, 2018, 2021, 2024}
    #[arg(short, long, default_value = "2015")]
    edition: Edition,
    /// rustfmt `style_edition` used when formatting code. Possible vales {2015, 2018, 2021, 2024}.
    #[arg(short, long, default_value = "2021")]
    style_edition: StyleEdition,
    /// Optional commit hash from the feature branch
    #[arg(short, long)]
    commit_hash: Option<String>,
    /// Optional comma separated list of rustfmt config options to
    /// pass when running the feature branch
    #[arg(value_delimiter = ',', short, long, num_args = 1..)]
    rustfmt_config: Option<Vec<String>>,
}

fn main() -> Result<ExitCode, Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("CHECK_DIFF_LOG"))
        .init();
    let args = CliInputs::parse();
    let tmp_dir = tempdir()?;
    info!("Created tmp_dir {:?}", tmp_dir);

    let compilation_result = compile_rustfmt(
        tmp_dir.path(),
        args.remote_repo_url,
        args.feature_branch,
        args.edition,
        args.style_edition,
        args.commit_hash,
    );

    let check_diff_runners = match compilation_result {
        Ok(runner) => runner,
        Err(e) => {
            error!("Failed to compile rustfmt:\n{e:?}");
            return Ok(ExitCode::FAILURE);
        }
    };

    let errors = Arc::new(AtomicUsize::new(0));
    let rustfmt_config = Arc::new(args.rustfmt_config);
    let check_diff_runners = Arc::new(check_diff_runners);

    thread::scope(|s| {
        for url in REPOS {
            let errors = Arc::clone(&errors);
            let rustfmt_config = Arc::clone(&rustfmt_config);
            let check_diff_runners = Arc::clone(&check_diff_runners);
            s.spawn(move || {
                let repo_name = get_repo_name(url);
                info!("Processing repo: {repo_name}");
                let Ok(tmp_dir) = tempdir() else {
                    warn!(
                        "Failed to create a tempdir for {}. Can't check formatting diff for {}",
                        &url, repo_name
                    );
                    return;
                };

                let Ok(_) = clone_git_repo(url, tmp_dir.path()) else {
                    warn!(
                        "Failed to clone repo {}. Can't check formatting diff for {}",
                        &url, repo_name
                    );
                    return;
                };

                let error_count = check_diff(
                    rustfmt_config.as_deref(),
                    &check_diff_runners,
                    tmp_dir.path(),
                    url,
                );

                errors.fetch_add(error_count as usize, Ordering::Relaxed);
            });
        }
    });

    let error_count = Arc::into_inner(errors)
        .expect("All other threads are done")
        .load(Ordering::Relaxed);
    if error_count > 0 {
        error!("Formatting diff found 💔");
        Ok(ExitCode::from(u8::try_from(error_count).unwrap_or(u8::MAX)))
    } else {
        info!("No diff found 😊");
        Ok(ExitCode::SUCCESS)
    }
}
