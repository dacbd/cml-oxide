mod types;

use crate::error::CmlError;
use crate::{common, Forge, ForgeType};
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
    pub async fn send(&self, forge: &Forge) -> Result<String> {
        let body = format!("{}\n\n{}", self.provided_body, self.watermark);
        let state = common::State::init().unwrap();

        let owner_str = state
            .git_remote
            .owner
            .ok_or_else(|| CmlError::Unimplemented)?;
        let owner = owner_str.as_str();
        let repo_name = state.git_remote.name.as_str();

        match &self.target {
            types::CommentTarget::Commit(id) => {
                let sha = if id.is_empty() { &state.head_sha } else { id };
                return forge.comment_on_commit(owner, repo_name, sha, body).await;
            }
            types::CommentTarget::Pr(id) => {
                let pr_number = if id.is_empty() {
                    forge
                        .get_pr_by_commit_sha(owner, repo_name, &state.head_sha)
                        .await?
                } else {
                    id.to_string()
                };
                return forge
                    .comment_on_pr(owner, repo_name, &pr_number, body)
                    .await;
            }
            types::CommentTarget::Issue(id) => {
                let issue_number = if id.is_empty() {
                    return Err(crate::error::CmlError::Unimplemented.into());
                } else {
                    id
                };
                return forge
                    .comment_on_issue(owner, repo_name, issue_number, body)
                    .await;
            }
        }
    }
}
