use check_diff::{CheckDiffError, check_diff, compile_rustfmt};
use clap::Parser;
use tempfile::Builder;
use tracing::info;

const REPOS: &[&str] = &[
    "https://github.com/rust-lang/rust.git",
    "https://github.com/rust-lang/cargo.git",
    "https://github.com/rust-lang/miri.git",
    "https://github.com/rust-lang/rust-analyzer.git",
    "https://github.com/bitflags/bitflags.git",
    "https://github.com/rust-lang/log.git",
    "https://github.com/rust-lang/mdBook.git",
    "https://github.com/rust-lang/packed_simd.git",
    "https://github.com/rust-lang/rust-semverver.git",
    "https://github.com/Stebalien/tempfile.git",
    "https://github.com/rust-lang/futures-rs.git",
    "https://github.com/dtolnay/anyhow.git",
    "https://github.com/dtolnay/thiserror.git",
    "https://github.com/dtolnay/syn.git",
    "https://github.com/serde-rs/serde.git",
    "https://github.com/rust-lang/rustlings.git",
    "https://github.com/rust-lang/rustup.git",
    "https://github.com/SergioBenitez/Rocket.git",
    "https://github.com/rustls/rustls.git",
    "https://github.com/rust-lang/rust-bindgen.git",
    "https://github.com/hyperium/hyper.git",
    "https://github.com/actix/actix.git",
    "https://github.com/denoland/deno.git",
];

/// Inputs for the check_diff script
#[derive(Parser)]
struct CliInputs {
    /// Git url of a rustfmt fork to compare against the latest main rustfmt
    remote_repo_url: String,
    /// Name of the feature branch on the forked repo
    feature_branch: String,
    /// Optional commit hash from the feature branch
    #[arg(short, long)]
    commit_hash: Option<String>,
    /// Optional comma separated list of rustfmt config options to
    /// pass when running the feature branch
    #[arg(value_delimiter = ',', short, long, num_args = 1..)]
    rustfmt_config: Option<Vec<String>>,
}

fn main() -> Result<(), CheckDiffError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("CHECK_DIFF_LOG"))
        .init();
    let args = CliInputs::parse();
    let tmp_dir = Builder::new().tempdir_in("").unwrap();
    info!("Created tmp_dir {:?}", tmp_dir);
    let check_diff_runners = compile_rustfmt(
        tmp_dir.path(),
        args.remote_repo_url,
        args.feature_branch,
        args.commit_hash,
    )?;

    // TODO: currently using same tmp dir path for sake of compilation
    let _ = check_diff(args.rustfmt_config, check_diff_runners, tmp_dir.path());

    Ok(())
}
