use clap::Parser;
use serde::Deserialize;
use std::fmt;
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
    #[arg(short, long, value_enum)]
    target: Option<String>,
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
enum CommentLocation {
    Commit(String),
    Pr(String),
    Issue(String),
}
impl FromStr for CommentLocation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "commit" => Ok(CommentLocation::Commit(String::from(""))),
            "pr" => Ok(CommentLocation::Pr(String::from(""))),
            "issue" => Ok(CommentLocation::Issue(String::from(""))),
            _ => {
                let parts = s.split_once("/");
                match parts {
                    Some((comment_type, id)) => {
                        let comment_id = String::from(id);
                        match comment_type {
                            "commit" => return Ok(CommentLocation::Commit(comment_id)),
                            "pr" => return Ok(CommentLocation::Pr(comment_id)),
                            "issue" => return Ok(CommentLocation::Issue(comment_id)),
                            _ => return Err("Unknown target".into()),
                        }
                    }
                    None => return Err("Invalid format".into()),
                }
            }
        }
    }
}
impl<'de> Deserialize<'de> for CommentLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        return match CommentLocation::from_str(s) {
            Ok(cl) => Ok(cl),
            Err(e) => Err(serde::de::Error::custom(e))
        }
    }
}
impl fmt::Display for CommentLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentLocation::Commit(id) => write!(f, "commit/{}", id),
            CommentLocation::Pr(id) => write!(f, "pr/{}", id),
            CommentLocation::Issue(id) => write!(f, "issue/{}", id),
        }
    }
}
impl CommentLocation {
    pub fn c_from_string(s: &String) -> Option<Self> {
        let parts = s.split_once("/");
        match parts {
            Some((comment_type, id)) => {
                let comment_id = String::from(id);
                match comment_type {
                    "commit" => return Some(CommentLocation::Commit(comment_id)),
                    "pr" => return Some(CommentLocation::Pr(comment_id)),
                    "issue" => return Some(CommentLocation::Issue(comment_id)),
                    _ => return None,
                }
            }
            None => return None,
        }
    }
    pub fn c_from_run_state(going_to_be_a_struct: String) -> Option<Self> {
        todo!("todo")
    }
}

#[derive(Debug)]
pub struct Comment {
    provided_body: String,
    watermark: String,
    target: CommentLocation,
}

impl Comment {
    pub fn from_args(args: &Args) -> Self {
        let target = match &args.target {
            Some(t) => ,
            // todo from state
            None => CommentLocation::Commit(String::from("todo")),
        };
        return Comment {
            provided_body: args.get_comment_body(),
            watermark: String::from("todo"),
            target,
        };
    }
}
