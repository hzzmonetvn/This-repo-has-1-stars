# This-repo-has-N-stars

> This repo has **0** stars, star it to test!

A high-performance, memory-safe, dynamic GitHub repository star synchronizer written in 100% Rust.

The repository name, description, and README dynamically synchronize whenever someone stars or unstars it.

## Features

- **100% Idiomatic Rust**: Built with `tokio`, `reqwest`, `clap`, `serde`, and `thiserror`.
- **Zero-Warning Strict Quality**: Passes `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt`.
- **Dual Execution Modes**:
  - `sync`: One-shot execution tailored for GitHub Actions / CI automation.
  - `daemon`: Continuous polling daemon for persistent server deployment.
- **Immutable Endpoint Targeting**: Operates on GitHub repository IDs (`/repositories/{id}`) rather than name-based URLs, eliminating redirect loops and race conditions during renames.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=hzzmonetvn/This-repo-has-0-stars&type=Date)](https://star-history.com/#hzzmonetvn/This-repo-has-0-stars&Date)

## CLI Usage

### Build

```bash
cargo build --release
```

### Run One-Shot Sync

```bash
export SYNC_TOKEN="your_github_pat"
cargo run --release -- sync
```

### Run Daemon Mode

```bash
export SYNC_TOKEN="your_github_pat"
export SYNC_INTERVAL_SECS=3
cargo run --release -- daemon --interval-secs 3
```

## Environment Variables

| Variable | Description | Default |
| --- | --- | --- |
| `SYNC_TOKEN` / `GITHUB_TOKEN` | GitHub Personal Access Token (`repo` scope) | Required |
| `REPO_ID` | GitHub repository ID (immutable across renames) | `1319484859` |
| `README_PATH` | Path to `README.md` file | `README.md` |
| `SYNC_INTERVAL_SECS` | Polling interval in daemon mode | `3` |
| `RUST_LOG` | Tracing logging level filter (`debug`, `info`, `warn`) | `info` |

## License

MIT - [hzzmonetvn](https://github.com/hzzmonetvn)
