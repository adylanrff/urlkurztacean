use std::fmt::Display;

use url;

#[derive(Debug, Clone)]
pub struct OriginalUrl(url::Url);

#[derive(Debug)]
pub enum UrlError {
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
