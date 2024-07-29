use serde::Serialize;
use tracing::error;

#[derive(Debug, clap::ValueEnum, Clone, Serialize)]
pub enum ForgeType {
    Github,
    Gitlab,
    Unknown,
}
impl ForgeType {
    pub fn get_token(&self) -> Result<String, &str> {
        let native_token: Option<String> = match self {
            Self::Github => std::env::var("GITHUB_TOKEN").ok(),
            Self::Gitlab => std::env::var("GITLAB_TOKEN").ok(),
            Self::Unknown => None,
        };
        match native_token {
            Some(t) => Ok(t),
            None => match std::env::var("CML_TOKEN").or_else(|_| std::env::var("REPO_TOKEN")) {
                Ok(t) => Ok(t),
                Err(e) => {
                    error!(error = ?e, "Failed to load auth env variables");
                    Err("No token detected")
                }
            },
        }
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
