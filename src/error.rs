use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmlError {
    #[error("Some unexpected state, that shouldn't be possible has been encounted.")]
    UnexpectedState,
    #[error("This feature has not been implemented yet.")]
    Unimplemented,
    #[error("Couldn't determine HEAD sha")]
    UnknownHeadSha,
    #[error("Couldn't determine git remote")]
    UnknownGitRemote,
    #[error("No Token Detected.")]
    NoAuthToken,
}
