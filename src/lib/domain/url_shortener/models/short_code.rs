use core::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ShortCode(String);

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
