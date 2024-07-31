use crate::error::CmlError;
use anyhow::Result;
use octocrab::Octocrab;
use serde::Serialize;
use tracing::error;

#[derive(Debug, clap::ValueEnum, Clone, Serialize)]
pub enum ForgeType {
    Github,
    Gitlab,
    Unknown,
}
impl ForgeType {
    pub fn get_token(&self) -> Result<String> {
        let native_token: Option<String> = match self {
            Self::Github => std::env::var("GITHUB_TOKEN").ok(),
            Self::Gitlab => std::env::var("GITLAB_TOKEN").ok(),
            Self::Unknown => None,
        };
        return match native_token {
            Some(t) => Ok(t),
            None => match std::env::var("CML_TOKEN").or_else(|_| std::env::var("REPO_TOKEN")) {
                Ok(t) => Ok(t),
                Err(e) => {
                    error!(error = ?e, "Failed to load auth env variables");
                    Err(CmlError::NoAuthToken.into())
                }
            },
        };
    }
}
impl Default for ForgeType {
    fn default() -> Self {
        if let Ok(token) = std::env::var("CML_TOKEN").or_else(|_| std::env::var("REPO_TOKEN")) {
            // TODO: inspect token to determine forge
            let gh_prefixes = ["ghp", "gho", "ghu", "ghs", "ghr"];
            if gh_prefixes.iter().any(|&prefix| token.starts_with(prefix)) {
                return Self::Github;
            }
            return Self::Unknown;
        }
        if let Ok(_github_token) = std::env::var("GITHUB_TOKEN") {
            return Self::Github;
        }
        if let Ok(_gitlab_token) = std::env::var("GITLAB_TOKEN") {
            return Self::Gitlab;
        }
        return Self::Unknown;
    }
}

pub struct Forge {
    forge_type: ForgeType,
    token: String,
    github: Option<Octocrab>,
    gitlab: Option<Octocrab>, // TODO: gitlab api client
}
impl Forge {
    pub fn init(forge_type: ForgeType) -> Result<Self> {
        let token = forge_type
            .get_token()
            .map_err(|_| CmlError::Unimplemented)?;
        match forge_type {
            ForgeType::Gitlab => {
                todo!("not implemented")
            }
            ForgeType::Github => Ok(Self {
                forge_type,
                token: token.to_owned(),
                github: Some(Octocrab::builder().personal_token(token).build()?),
                gitlab: None,
            }),
            ForgeType::Unknown => Err(CmlError::Unimplemented.into()),
        }
    }
    pub async fn comment_on_pr(
        &self,
        owner: &str,
        repo: &str,
        id: &String,
        body: String,
    ) -> Result<String> {
        match self.forge_type {
            ForgeType::Unknown => Err(CmlError::Unimplemented.into()),
            ForgeType::Gitlab => Err(CmlError::Unimplemented.into()),
            ForgeType::Github => return self.comment_on_issue(owner, repo, id, body).await,
        }
    }
    pub async fn comment_on_commit(
        &self,
        owner: &str,
        repo: &str,
        sha: &String,
        body: String,
    ) -> Result<String> {
        match self.forge_type {
            ForgeType::Unknown => Err(CmlError::Unimplemented.into()),
            ForgeType::Gitlab => Err(CmlError::Unimplemented.into()),
            ForgeType::Github => {
                let comment = self
                    .github
                    .as_ref()
                    .ok_or_else(|| CmlError::UnexpectedState)?
                    .commits(owner, repo)
                    .create_comment(sha, body)
                    .send()
                    .await?;
                return Ok(comment.html_url.to_string());
            }
        }
    }
    pub async fn comment_on_issue(
        &self,
        owner: &str,
        repo: &str,
        id: &String,
        body: String,
    ) -> Result<String> {
        match self.forge_type {
            ForgeType::Unknown => Err(CmlError::Unimplemented.into()),
            ForgeType::Gitlab => Err(CmlError::Unimplemented.into()),
            ForgeType::Github => {
                let issue_id: u64 = u64::from_str_radix(&id, 10)?;
                let comment = self
                    .github
                    .as_ref()
                    .ok_or_else(|| CmlError::UnexpectedState)?
                    .issues(owner, repo)
                    .create_comment(issue_id, body)
                    .await?;
                return Ok(comment.html_url.to_string());
            }
        }
    }
    // helper functions
    pub async fn get_pr_by_commit_sha(&self, owner: &str, repo: &str, sha: &str) -> Result<String> {
        match self.forge_type {
            ForgeType::Unknown => Err(CmlError::Unimplemented.into()),
            ForgeType::Gitlab => Err(CmlError::Unimplemented.into()),
            ForgeType::Github => {
                let pr_id = self
                    .github
                    .as_ref()
                    .ok_or_else(|| CmlError::UnexpectedState)?
                    .commits(owner, repo)
                    .associated_pull_requests(octocrab::commits::PullRequestTarget::Sha(
                        sha.to_string(),
                    ))
                    .send()
                    .await?
                    .take_items()
                    .last()
                    .ok_or(CmlError::UnexpectedState)?
                    .number
                    .to_string();
                return Ok(pr_id);
            }
        }
    }
}
