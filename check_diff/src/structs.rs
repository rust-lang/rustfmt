use clap::Parser;

/// Inputs for the check_diff script
#[derive(Parser)]
pub struct CliInputs {
    /// Git url of a rustfmt fork to compare against the latest master rustfmt
    pub remote_repo_url: String,
    /// Name of the feature branch on the forked repo
    pub feature_branch: String,
    /// Optional commit hash from the feature branch
    #[arg(short, long)]
    pub commit_hash: Option<String>,
    /// Optional comma separated list of rustfmt config options to
    /// pass when running the feature branch
    #[arg(value_delimiter = ',', short, long, num_args = 1..)]
    pub rustfmt_config: Option<Vec<String>>,
}
