pub mod config;
pub mod error;
pub mod github;
pub mod readme;

use crate::error::SyncError;
use crate::github::GitHubClient;
use std::path::Path;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncSummary {
    pub stars: u64,
    pub repo_name: String,
    pub name_updated: bool,
    pub readme_updated: bool,
}

/// Execute a single synchronization pass across GitHub metadata and README file.
pub async fn run_sync_pass(
    client: &GitHubClient,
    readme_path: &Path,
) -> Result<SyncSummary, SyncError> {
    let repo_info = client.fetch_repo_info().await?;
    let stars = repo_info.stars;
    let target_name = format!("This-repo-has-{stars}-stars");
    let target_desc = format!("This repo has {stars} stars, star it to test!");

    info!(
        current_name = %repo_info.name,
        target_name = %target_name,
        stars = stars,
        "evaluating synchronization state"
    );

    let needs_name_update =
        repo_info.name != target_name || repo_info.description.as_deref() != Some(&target_desc);

    let name_updated = if needs_name_update {
        client.update_repo(&target_name, &target_desc).await?
    } else {
        info!("repository name and description are already up to date");
        false
    };

    let readme_updated = readme::update_readme_file(readme_path, stars, &target_name)?;

    Ok(SyncSummary {
        stars,
        repo_name: target_name,
        name_updated,
        readme_updated,
    })
}
