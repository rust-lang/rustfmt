use check_diff::{compile_rustfmt, structs::CliInputs};
use clap::Parser;
use tempfile::Builder;
use tracing::info;

fn main() {
    let args = CliInputs::parse();
    let tmp_dir = Builder::new().tempdir_in("").unwrap();
    info!("Created tmp_dir {:?}", tmp_dir);
    let _ = compile_rustfmt(tmp_dir.path(), args);
}
