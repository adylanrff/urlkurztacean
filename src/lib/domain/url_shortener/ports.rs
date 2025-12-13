use async_trait::async_trait;
use thiserror::Error;

use crate::domain::url_shortener::models::{short_code::ShortCode, shortened_url::ShortenedUrl};

#[derive(Debug, Error)]
pub enum CreateUrlError {
    #[error("database error")]
    DBError(anyhow::Error),
    #[error("url already exists")]
    AlreadyExists,
    #[error("lock error")]
    LockError,
}

#[derive(Debug, Error)]
pub enum GetByCodeError {
    #[error("database error")]
    DBError(anyhow::Error),
    #[error("url not found")]
    NotFound,
}

#[async_trait]
pub trait ShortenedUrlRepository {
    async fn create(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError>;
    async fn get_by_code(&self, code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError>;
}
