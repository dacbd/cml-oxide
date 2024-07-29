mod types;

use crate::error::CmlError;
use crate::{common, ForgeType};
use anyhow::Result;
use clap::Parser;
use octocrab;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command()]
pub struct Args {
    /// Path to the file which will be comment's body/text
    file: String,
    /// Comment type, ex: commit, pr, issue, commit/1337dacb, pr/42, issue/4237
    /// default is pr, but will fall back to a commit comment
    #[arg(short, long, value_parser = types::CommentTarget::from_str )]
    target: Option<types::CommentTarget>,
    #[arg(long, default_value_t = true)]
    watermark: bool,
}
impl Args {
    fn path_is_file(&self) -> bool {
        let path = Path::new(&self.file);
        return path.exists() && path.is_file();
    }
    fn read_file(&self) -> String {
        fs::read_to_string(&self.file).expect("valid path")
    }
    pub fn get_comment_body(&self) -> String {
        if self.path_is_file() {
            return self.read_file();
        }
        return self.file.to_owned();
    }
}

#[derive(Debug)]
pub struct Comment {
    provided_body: String,
    watermark: String,
    target: types::CommentTarget,
}

impl Comment {
    pub fn from_args(args: &Args) -> Self {
        let target = match &args.target {
            Some(t) => t.to_owned(),
            // todo from state
            None => types::CommentTarget::Commit(String::from("todo")),
        };
        return Comment {
            provided_body: args.get_comment_body(),
            watermark: String::from("todo"),
            target,
        };
    }
    pub async fn send(&self, forge: ForgeType) -> Result<String> {
        let body = format!("{}\n\n{}", self.provided_body, self.watermark);
        let state = common::State::init().unwrap();
        let pat = forge.get_token().unwrap();

        let github = octocrab::Octocrab::builder()
            .personal_token(pat)
            .build()
            .unwrap();

        let owner = &state.git_remote.owner.unwrap();
        let repo_name = &state.git_remote.name;

        match &self.target {
            types::CommentTarget::Commit(id) => {
                let sha = if id.is_empty() { &state.head_sha } else { id };
                let comment = github
                    .commits(owner, repo_name)
                    .create_comment(sha, body)
                    .send()
                    .await?;
                return Ok(comment.html_url.to_string());
            }
            types::CommentTarget::Pr(id) => {
                let pr_number = if id.is_empty() {
                    github
                        .commits(owner, repo_name)
                        .associated_pull_requests(octocrab::commits::PullRequestTarget::Sha(
                            state.head_sha,
                        ))
                        .send()
                        .await?
                        .take_items()
                        .last()
                        .ok_or(crate::error::CmlError::Unimplemented)?
                        .number
                        .to_string()
                } else {
                    id.to_string()
                };
                let pr_id = u64::from_str(&pr_number)?;
                let comment = github
                    .issues(owner, repo_name)
                    .create_comment(pr_id, body)
                    .await?;
                return Ok(comment.html_url.to_string());
            }
            types::CommentTarget::Issue(id) => {
                let issue_number = if id.is_empty() {
                    return Err(crate::error::CmlError::Unimplemented.into());
                } else {
                    u64::from_str(id)?
                };
                let comment = github
                    .issues(owner, repo_name)
                    .create_comment(issue_number, body)
                    .await?;
                return Ok(comment.html_url.to_string());
            }
        }
    }
}
