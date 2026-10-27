use clap::Parser;
use std::env;
use std::time::Duration;
use this_repo_has_stars::config::{Cli, Commands};
use this_repo_has_stars::error::SyncError;
use this_repo_has_stars::github::GitHubClient;
use this_repo_has_stars::run_sync_pass;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    let token = cli
        .token
        .or_else(|| env::var("GITHUB_TOKEN").ok())
        .filter(|t| !t.trim().is_empty())
        .ok_or(SyncError::MissingToken)?;

    let client = GitHubClient::new(&cli.repo_id, &token)?;

    match cli.command {
        Commands::Sync => {
            info!("running in one-shot sync mode");
            let summary = run_sync_pass(&client, &cli.readme_path).await?;
            info!(
                stars = summary.stars,
                repo_name = %summary.repo_name,
                name_updated = summary.name_updated,
                readme_updated = summary.readme_updated,
                "sync completed successfully"
            );
        }
        Commands::Daemon(daemon_args) => {
            info!(
                interval_secs = daemon_args.interval_secs,
                "starting star sync daemon loop"
            );
            let mut interval =
                tokio::time::interval(Duration::from_secs(daemon_args.interval_secs));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        match run_sync_pass(&client, &cli.readme_path).await {
                            Ok(summary) => {
                                if summary.name_updated || summary.readme_updated {
                                    info!(
                                        stars = summary.stars,
                                        repo_name = %summary.repo_name,
                                        "synced star changes"
                                    );
                                }
                            }
                            Err(e) => {
                                warn!(error = %e, "sync pass encountered error, will retry on next tick");
                            }
                        }
                    }
                    _ = tokio::signal::ctrl_c() => {
                        info!("shutdown signal received, exiting daemon loop cleanly");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
