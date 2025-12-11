use thiserror::Error;

use crate::domain::url_shortener::models::{short_code::ShortCode, shortened_url::ShortenedUrl};

#[derive(Debug, Error)]
pub enum CreateUrlError {
    #[error("database error")]
    DBError(anyhow::Error),
}

#[derive(Debug, Error)]
pub enum GetByCodeError {
    #[error("database error")]
    DBError(anyhow::Error),
    #[error("url not found")]
    NotFound,
}

pub trait UrlRepository {
    fn create(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError>;
    fn get_by_code(&self, code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError>;
}
