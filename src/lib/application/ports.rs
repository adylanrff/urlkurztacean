use std::fmt::Debug;

use async_trait::async_trait;
use thiserror::Error;

use crate::domain::url_shortener::models::{short_code::ShortCode, shortened_url::ShortenedUrl};

#[derive(Debug, Error)]
pub enum CreateUrlError {
    #[error("database error")]
    DBError(#[from] sqlx::Error),
    #[error("url already exists")]
    AlreadyExists,
    #[error("lock error")]
    LockError,
}

#[derive(Debug, Error)]
pub enum GetByCodeError {
    #[error("database error")]
    DBError(#[from] sqlx::Error),
    #[error("url not found")]
    NotFound,
}

#[async_trait]
pub trait ShortenedUrlRepository {
    async fn create(&self, shortened_url: &ShortenedUrl) -> Result<ShortenedUrl, CreateUrlError>;
    async fn get_by_code(&self, code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError>;
}

#[async_trait]
pub trait UrlShortenerService: Clone + Send + Sync + 'static + Debug {
    async fn shorten_url(
        &self,
        url: impl Into<String> + Send,
    ) -> Result<ShortenedUrl, anyhow::Error>;

    async fn get_by_code(
        &self,
        code: impl Into<String> + Send,
    ) -> Result<ShortenedUrl, anyhow::Error>;
}
