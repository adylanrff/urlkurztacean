use core::fmt::Display;

#[derive(Debug, Clone)]
pub struct ShortCode(String);

#[derive(Debug)]
pub enum ShortCodeError {
    NoEmptyString,
}

impl ShortCode {
    pub fn new(e: &str) -> Result<Self, ShortCodeError> {
        if e.is_empty() {
            Err(ShortCodeError::NoEmptyString)
        } else {
            Ok(ShortCode(e.to_string()))
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
