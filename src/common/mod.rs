use std::env;

use git2::Repository;
use tracing::error;

pub struct GitMetadata {
    repo: Repository,
    head_sha: String,
}

pub fn cwd_is_git_repo() -> Option<Repository> {
    match env::current_dir() {
        Ok(path) => match Repository::open(path) {
            Ok(repo) => Some(repo),
            Err(e) => {
                error!(error = ?e, "Failed to open git repo");
                return None;
            }
        },
        Err(e) => {
            error!(error = ?e, "Failed to determine current dir");
            return None;
        }
    }
}
