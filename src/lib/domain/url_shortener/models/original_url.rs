use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;
use url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginalUrl(url::Url);

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("parse error: {0}")]
    ParseError(url::ParseError),
}

impl OriginalUrl {
    pub fn new(input: &str) -> Result<Self, UrlError> {
        match url::Url::parse(input) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(UrlError::ParseError(e)),
        }
    }
}

impl Display for OriginalUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
