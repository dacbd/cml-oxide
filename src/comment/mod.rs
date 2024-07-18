mod types;
use clap::Parser;
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
}
