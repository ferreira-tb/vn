use reqwest::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("client is closed")]
  ClientClosed,

  #[error("\"{0}\" is not a valid id")]
  InvalidId(String),

  #[error("failed to parse JSON: {0}")]
  Json(#[from] serde_json::Error),

  #[error("{}", reqwest_error(*status, reason))]
  RequestFailed {
    status: Option<StatusCode>,
    reason: String,
  },

  #[error("not authorized: token needed")]
  TokenNeeded,
}

impl Error {
  pub fn is_token_needed(&self) -> bool {
    matches!(self, Self::TokenNeeded)
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    let status = error.status();
    let reason = error.to_string();
    Self::RequestFailed { status, reason }
  }
}

fn reqwest_error(status: Option<StatusCode>, reason: &str) -> String {
  match status {
    Some(status) => format!("[{status}] {reason}"),
    None => format!("request failed: {reason}"),
  }
}
