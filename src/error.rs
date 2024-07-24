use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmlError {
    #[error("This feature has not been implemented yet.")]
    Unimplemented,
    #[error("Couldn't determine HEAD sha")]
    UnknownHeadSha,
    #[error("Couldn't determine git remote")]
    UnknownGitRemote,
}
