use std::env;

use crate::error::CmlError;
use anyhow::Result;
use git2::{Config, Repository};
use git_url_parse::GitUrl;
use tracing::{error, info};

pub struct State {
    pub head_sha: String,
    pub git_remote: GitUrl,
}
impl State {
    // private functions
    fn open_repo() -> Result<Repository> {
        match env::current_dir() {
            Ok(path) => return Ok(Repository::open(path)?),
            Err(e) => {
                error!(error = ?e, "Failed to determine current dir");
                return Err(e.into());
            }
        }
    }
    fn get_remote(config: &Config) -> Result<GitUrl> {
        let mut entries = config.entries(Some("remote.*.url"))?;
        while let Some(entry) = entries.next() {
            let entry = entry?;
            let name = entry.name().ok_or(CmlError::UnknownGitRemote)?;
            let remote_url = entry.value().ok_or(CmlError::UnknownGitRemote)?;
            info!(name = name, value = remote_url, "Found remote entry");
            match GitUrl::parse(remote_url) {
                Ok(url) => return Ok(url),
                Err(_) => {}
            }
        }
        return Err(CmlError::UnknownGitRemote.into());
    }
    // pub functions
    pub fn init() -> Result<Self> {
        let repo = State::open_repo()?;
        let git_config = repo.config()?.snapshot()?;
        let head = repo.head()?;
        let sha = head.shorthand().ok_or(CmlError::UnknownHeadSha)?;
        let git_remote = Self::get_remote(&git_config)?;

        println!("{:?}", git_remote);
        return Ok(Self {
            head_sha: String::from(sha),
            git_remote,
        });
    }
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
