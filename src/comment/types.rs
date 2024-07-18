use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentTarget {
    Commit(String),
    Pr(String),
    Issue(String),
}
impl FromStr for CommentTarget {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "commit" => Ok(CommentTarget::Commit(String::from(""))),
            "pr" => Ok(CommentTarget::Pr(String::from(""))),
            "issue" => Ok(CommentTarget::Issue(String::from(""))),
            _ => {
                let parts = s.split_once("/");
                match parts {
                    Some((comment_type, id)) => {
                        let comment_id = String::from(id);
                        match comment_type {
                            "commit" => return Ok(CommentTarget::Commit(comment_id)),
                            "pr" => return Ok(CommentTarget::Pr(comment_id)),
                            "issue" => return Ok(CommentTarget::Issue(comment_id)),
                            _ => return Err("Unknown target".into()),
                        }
                    }
                    None => return Err("Invalid format".into()),
                }
            }
        }
    }
}
impl<'de> Deserialize<'de> for CommentTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        return match CommentTarget::from_str(s) {
            Ok(cl) => Ok(cl),
            Err(e) => Err(serde::de::Error::custom(e)),
        };
    }
}
impl fmt::Display for CommentTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentTarget::Commit(id) => write!(f, "commit/{}", id),
            CommentTarget::Pr(id) => write!(f, "pr/{}", id),
            CommentTarget::Issue(id) => write!(f, "issue/{}", id),
        }
    }
}
impl CommentTarget {
    pub fn c_from_string(s: &String) -> Option<Self> {
        let parts = s.split_once("/");
        match parts {
            Some((comment_type, id)) => {
                let comment_id = String::from(id);
                match comment_type {
                    "commit" => return Some(CommentTarget::Commit(comment_id)),
                    "pr" => return Some(CommentTarget::Pr(comment_id)),
                    "issue" => return Some(CommentTarget::Issue(comment_id)),
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

#[cfg(test)]
mod test_comment_target {
    use super::*;

    #[test]
    fn commit_parsing() {
        let input = "commit/deadbeef";
        let expected = CommentTarget::Commit("deadbeef".to_string());
        let result = CommentTarget::from_str(input);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn pr_parsing() {
        let input = "pr/123";
        let expected = CommentTarget::Pr("123".to_string());
        let result = CommentTarget::from_str(input);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn issue_parsing() {
        let input = "issue/456";
        let expected = CommentTarget::Issue("456".to_string());
        let result = CommentTarget::from_str(input);
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn invalid_format() {
        let input = "invalid/format";
        let result = CommentTarget::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn plain_word() {
        let commit = CommentTarget::from_str("commit");
        assert_eq!(commit, Ok(CommentTarget::Commit("".to_string())));
        let pr = CommentTarget::from_str("pr");
        assert_eq!(pr, Ok(CommentTarget::Pr("".to_string())));
        let issue = CommentTarget::from_str("issue");
        assert_eq!(issue, Ok(CommentTarget::Issue("".to_string())));
    }

    #[test]
    fn trailing_slashes() {
        let commit = CommentTarget::from_str("commit/");
        assert_eq!(commit, Ok(CommentTarget::Commit("".to_string())));
        let pr = CommentTarget::from_str("pr/");
        assert_eq!(pr, Ok(CommentTarget::Pr("".to_string())));
        let issue = CommentTarget::from_str("issue/");
        assert_eq!(issue, Ok(CommentTarget::Issue("".to_string())));
    }
}
