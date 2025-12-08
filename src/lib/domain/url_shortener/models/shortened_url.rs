use std::fmt::Display;

use crate::domain::url_shortener::models::{original_url::OriginalUrl, short_code::ShortCode};

#[derive(Debug, Clone)]
pub struct ShortenedUrl {
    original_url: OriginalUrl,
    short_code: ShortCode,
}

impl ShortenedUrl {
    pub fn new(original_url: OriginalUrl, short_code: ShortCode) -> Self {
        Self {
            original_url,
            short_code,
        }
    }
}

impl Display for ShortenedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.original_url, self.short_code)
    }
}
