use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    application::ports::{CreateUrlError, GetByCodeError, ShortenedUrlRepository},
    domain::url_shortener::models::{
        original_url::OriginalUrl, short_code::ShortCode, shortened_url::ShortenedUrl,
    },
};

#[derive(Clone, Debug)]
pub struct PostgresStorage {
    pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
struct PostgresShortenedUrlRecord {
    original_url: String,
    short_code: String,
}

impl From<PostgresShortenedUrlRecord> for ShortenedUrl {
    fn from(value: PostgresShortenedUrlRecord) -> Self {
        let original_url = OriginalUrl::new(&value.original_url).unwrap();
        let short_code = ShortCode::new(&value.short_code).unwrap();

        Self {
            original_url: original_url,
            short_code: short_code,
        }
    }
}

#[async_trait]
impl ShortenedUrlRepository for PostgresStorage {
    async fn create(&self, shortened_url: &ShortenedUrl) -> Result<ShortenedUrl, CreateUrlError> {
        sqlx::query!(
            "INSERT INTO shortenedurl(original_url,short_code) VALUES ($1,$2)",
            shortened_url.original_url.to_string(),
            shortened_url.short_code.to_string(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                CreateUrlError::AlreadyExists
            }
            other => CreateUrlError::DBError(other),
        })?;

        return Ok(ShortenedUrl::new(
            shortened_url.original_url.clone(),
            shortened_url.short_code.clone(),
        ));
    }

    async fn get_by_code(&self, short_code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError> {
        let shortened_url = sqlx::query_as!(
            PostgresShortenedUrlRecord,
            "SELECT * from shortenedurl WHERE short_code=$1",
            short_code.to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => GetByCodeError::NotFound,
            other => GetByCodeError::DBError(other),
        })?;

        Ok(shortened_url.into())
    }
}

impl PostgresStorage {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
