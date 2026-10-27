use crate::error::SyncError;
use regex::Regex;
use std::fs;
use std::path::Path;
use tracing::info;

/// Update star counts and repository references within README content.
pub fn update_readme_content(content: &str, stars: u64, target_name: &str) -> (String, bool) {
    let re_repo = Regex::new(r"This-repo-has-\d+-stars").expect("valid regex pattern");
    let re_bold = Regex::new(r"has \*\*\d+\*\* stars").expect("valid regex pattern");
    let re_plain = Regex::new(r"has \d+ stars").expect("valid regex pattern");

    let step1 = re_repo.replace_all(content, target_name);
    let bold_replacement = format!("has **{stars}** stars");
    let step2 = re_bold.replace_all(&step1, bold_replacement.as_str());
    let plain_replacement = format!("has {stars} stars");
    let final_content = re_plain
        .replace_all(&step2, plain_replacement.as_str())
        .into_owned();

    let changed = final_content != content;
    (final_content, changed)
}

/// Read, update, and write README.md if content has changed.
pub fn update_readme_file(path: &Path, stars: u64, target_name: &str) -> Result<bool, SyncError> {
    if !path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(path).map_err(|e| SyncError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let (updated_content, changed) = update_readme_content(&content, stars, target_name);

    if changed {
        fs::write(path, updated_content).map_err(|e| SyncError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        info!(path = ?path, stars = stars, "updated readme file with new star count");
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_readme_content_changes() {
        let input = r#"# This-repo-has-1426-stars
> This repo has **1426** stars, star it to test!
This repository currently has 1426 stars.
[![Star Chart](https://api.star-history.com/svg?repos=hzzmonetvn/This-repo-has-1426-stars&type=Date)]"#;

        let (output, changed) = update_readme_content(input, 1, "This-repo-has-1-stars");
        assert!(changed);
        assert!(output.contains("# This-repo-has-1-stars"));
        assert!(output.contains("> This repo has **1** stars"));
        assert!(output.contains("This repository currently has 1 stars."));
        assert!(output.contains("repos=hzzmonetvn/This-repo-has-1-stars&type=Date"));
    }

    #[test]
    fn test_update_readme_content_idempotent() {
        let input = r#"# This-repo-has-1-stars
> This repo has **1** stars, star it to test!
This repository currently has 1 stars."#;

        let (output, changed) = update_readme_content(input, 1, "This-repo-has-1-stars");
        assert!(!changed);
        assert_eq!(output, input);
    }
}
