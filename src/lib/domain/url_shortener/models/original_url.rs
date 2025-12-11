use std::fmt::Display;
use thiserror::Error;

use url;

#[derive(Debug, Clone)]
pub struct OriginalUrl(url::Url);

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("url parsing error")]
    ParseError(#[from] url::ParseError),
}

impl OriginalUrl {
    pub fn new(input: &str) -> Result<Self, UrlError> {
        Ok(OriginalUrl(url::Url::parse(input)?))
    }
}

impl Display for OriginalUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
