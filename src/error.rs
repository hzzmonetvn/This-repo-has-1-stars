use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during star synchronization operations.
#[derive(Debug, Error)]
pub enum SyncError {
    #[error("missing required token in SYNC_TOKEN or GITHUB_TOKEN")]
    MissingToken,

    #[error("network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("github api returned error status {status}: {message}")]
    GitHubApi {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("failed to parse json response: {0}")]
    Json(#[from] serde_json::Error),

    #[error("file io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid repository id format: {0}")]
    InvalidRepoId(String),
}
