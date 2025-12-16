use core::fmt::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ShortCode(String);

impl Into<String> for ShortCode {
    fn into(self) -> String {
        return self.0;
    }
}

#[derive(Debug, Error)]
pub enum ShortCodeError {
    #[error("code cannot be empty")]
    NoEmptyString,
}

impl ShortCode {
    pub fn new(e: impl Into<String>) -> Result<Self, ShortCodeError> {
        let e = e.into();
        if e.is_empty() {
            Err(ShortCodeError::NoEmptyString)
        } else {
            Ok(ShortCode(e))
        }
    }
}

impl AsRef<str> for ShortCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ShortCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
