use std::fmt::Formatter;

#[derive(Debug)]
pub enum GithubError {
    Http(ureq::Error),
    JsonDeserialization(std::io::Error),
    RepositoryOrReleaseNotFound,
}

impl GithubError {
    pub fn from(error: ureq::Error) -> Self {
        match error {
            ureq::Error::Status(404, _) => Self::RepositoryOrReleaseNotFound,
            ureq::Error::Status(_, _) => Self::Http(error),
            ureq::Error::Transport(_) => Self::Http(error),
        }
    }
}

impl std::fmt::Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubError::Http(e) => f.write_str(&e.to_string()),
            GithubError::JsonDeserialization(e) => {
                f.write_str(&format!("Error deserializing response: {}", e))
            }
            GithubError::RepositoryOrReleaseNotFound => {
                f.write_str("Repository or release not found")
            }
        }
    }
}
