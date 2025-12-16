use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{
    application::ports::{CreateUrlError, GetByCodeError, ShortenedUrlRepository},
    domain::url_shortener::models::{
        original_url::OriginalUrl, short_code::ShortCode, shortened_url::ShortenedUrl,
    },
};

#[derive(Clone)]
pub struct PostgresStorage {
    pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
struct PostgresShortenedUrlRecord {
    original_url: String,
    short_code: String,
}

impl Into<ShortenedUrl> for PostgresShortenedUrlRecord {
    fn into(self) -> ShortenedUrl {
        let original_url = OriginalUrl::new(&self.original_url).unwrap();
        let short_code = ShortCode::new(&self.short_code).unwrap();
        ShortenedUrl::new(original_url, short_code)
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
        .await?;

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
        .await?;

        Ok(shortened_url.into())
    }
}

impl PostgresStorage {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
