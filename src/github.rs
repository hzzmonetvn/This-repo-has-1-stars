use crate::error::SyncError;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Clone, Deserialize)]
pub struct RepoInfo {
    #[serde(rename = "stargazers_count")]
    pub stars: u64,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
struct UpdateRepoPayload<'a> {
    name: &'a str,
    description: &'a str,
}

#[derive(Debug, Clone)]
pub struct GitHubClient {
    client: Client,
    api_url: String,
}

impl GitHubClient {
    /// Create a new GitHub API client for the given repository ID and token.
    pub fn new(repo_id: &str, token: &str) -> Result<Self, SyncError> {
        if token.trim().is_empty() {
            return Err(SyncError::MissingToken);
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("hzzmonetvn-star-sync-rust"),
        );

        let auth_val = format!("Bearer {}", token.trim());
        let mut auth_header =
            HeaderValue::from_str(&auth_val).map_err(|_| SyncError::GitHubApi {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: "invalid characters in authorization token".to_string(),
            })?;
        auth_header.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_header);

        let client = Client::builder().default_headers(headers).build()?;

        let api_url = format!("https://api.github.com/repositories/{repo_id}");

        Ok(Self { client, api_url })
    }

    /// Fetch repository metadata and current star count.
    pub async fn fetch_repo_info(&self) -> Result<RepoInfo, SyncError> {
        debug!(url = %self.api_url, "fetching repository info");
        let response = self.client.get(&self.api_url).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(SyncError::GitHubApi { status, message });
        }

        let repo_info: RepoInfo = response.json().await?;
        debug!(
            name = %repo_info.name,
            stars = repo_info.stars,
            "fetched repository info"
        );
        Ok(repo_info)
    }

    /// Update repository name and description if different from expected values.
    pub async fn update_repo(
        &self,
        target_name: &str,
        target_desc: &str,
    ) -> Result<bool, SyncError> {
        info!(
            target_name = %target_name,
            target_desc = %target_desc,
            "patching repository metadata"
        );

        let payload = UpdateRepoPayload {
            name: target_name,
            description: target_desc,
        };

        let response = self
            .client
            .patch(&self.api_url)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(SyncError::GitHubApi { status, message });
        }

        info!("successfully updated repository name and description on github");
        Ok(true)
    }
}
