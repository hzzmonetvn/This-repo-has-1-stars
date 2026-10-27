use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Dynamic GitHub repository star synchronizer.
#[derive(Debug, Parser)]
#[command(
    name = "this-repo-has-stars",
    version,
    about = "High-performance dynamic GitHub repository star synchronizer",
    author = "hzzmonetvn"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// GitHub Personal Access Token (PAT) with repo scope.
    #[arg(long, env = "SYNC_TOKEN", global = true)]
    pub token: Option<String>,

    /// GitHub repository ID (immutable across renames).
    #[arg(long, env = "REPO_ID", default_value = "1319484859", global = true)]
    pub repo_id: String,

    /// Path to README.md file.
    #[arg(long, env = "README_PATH", default_value = "README.md", global = true)]
    pub readme_path: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Perform a single synchronization pass (ideal for CI/CD and GitHub Actions).
    Sync,

    /// Run as a daemon process polling for star changes continuously.
    Daemon(DaemonArgs),
}

#[derive(Debug, Args)]
pub struct DaemonArgs {
    /// Polling interval in seconds between star checks.
    #[arg(
        short,
        long,
        env = "SYNC_INTERVAL_SECS",
        default_value_t = 3,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub interval_secs: u64,
}
