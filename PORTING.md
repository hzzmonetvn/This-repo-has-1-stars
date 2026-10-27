# Porting Notes: Multi-Language & Python Scripts to Idiomatic Rust

## 1. API & Architecture Mapping

| Original (Python / Legacy Scripts) | Ported Rust Implementation (`this-repo-has-stars`) | Architectural Rationale |
|---|---|---|
| `python/main.py`: `get_stars()` | `GitHubClient::fetch_repo_info(&self) -> Result<RepoInfo, SyncError>` | Replaced untyped dictionary parsing with strongly-typed `RepoInfo` struct (`serde`). |
| `python/main.py`: `update_project_name(data)` | `GitHubClient::update_repo(&self, name, desc) -> Result<bool, SyncError>` | Safe JSON serialization, explicit error propagation without panics. |
| `.github/scripts/sync.py`: README regex update | `readme::update_readme_content` & `readme::update_readme_file` | Pure, idempotent function with automated test coverage. |
| Hardcoded polling loop in scripts | `clap` CLI Subcommands: `Sync` (single-pass CI) & `Daemon` (async loop with graceful `SIGINT` handling) | Unifies one-shot GitHub Actions usage with persistent server daemon operation. |
| Global mutable token / URL | `config::Cli` struct parsing from flags and environment variables | Explicit configuration without global mutable state. |

## 2. Test Mapping

| Original Test | Rust Unit Test | Coverage Scope |
|---|---|---|
| No tests in original legacy scripts | `readme::tests::test_update_readme_content_changes` | Verifies regex matching for title, quote, text, and SVG chart links. |
| No tests in original legacy scripts | `readme::tests::test_update_readme_content_idempotent` | Verifies that running sync when counts match causes zero file churn. |

## 3. Deliberate Differences

1. **Immutable Repository ID vs Name Endpoint:**
   - *Original:* Legacy scripts used `PATCH /repos/{owner}/{name}`, causing 301/307 redirects or failures during consecutive renames.
   - *Rust Port:* Uses immutable repository ID `https://api.github.com/repositories/{id}` (`1319484859`).
2. **Error Handling:**
   - *Original:* Relied on print statements or unhandled network exceptions.
   - *Rust Port:* Implements a comprehensive `SyncError` enum with `thiserror`, mapping network errors, API status codes, and I/O failures.
3. **Structured Tracing:**
   - *Original:* Ad-hoc file logging or print statements.
   - *Rust Port:* Structured logging via `tracing` and `tracing-subscriber` configurable via `RUST_LOG`.

## 4. Preserved Quirks

- The repository naming pattern `This-repo-has-{N}-stars` and description format `This repo has {N} stars, star it to test!` remain strictly identical to preserve compatibility with existing star badges and community expectations.

## 5. Verification Commands & Results

- `cargo fmt --check`: Clean (0 diffs).
- `cargo clippy --all-targets --all-features -- -D warnings`: Clean (0 warnings).
- `cargo test`: 2 passed; 0 failed; 0 ignored.
- `cargo build --release`: Successful release artifact compiled.
