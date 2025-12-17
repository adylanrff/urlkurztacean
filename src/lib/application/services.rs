use std::fmt::Debug;

use async_trait::async_trait;
use nanoid::nanoid;

use crate::application::ports::ShortenedUrlRepository;
use crate::application::ports::UrlShortenerService;
use crate::domain::url_shortener::models::original_url::OriginalUrl;
use crate::domain::url_shortener::models::short_code::ShortCode;
use crate::domain::url_shortener::models::shortened_url::ShortenedUrl;

#[derive(Clone, Debug)]
pub struct Service<T: ShortenedUrlRepository> {
    repo: T,
}

const DEFAULT_SHORTCODE_LENGTH: usize = 8;

#[async_trait]
impl<T> UrlShortenerService for Service<T>
where
    T: ShortenedUrlRepository + Clone + Send + Sync + 'static + Debug,
{
    async fn shorten_url(
        &self,
        url: impl Into<String> + Send,
    ) -> Result<ShortenedUrl, anyhow::Error> {
        let original_url = OriginalUrl::new(url.into().as_str())?;
        let short_code = ShortCode::new(nanoid!(DEFAULT_SHORTCODE_LENGTH))?;
        let shortened_url = ShortenedUrl::new(original_url, short_code);

        Ok(self.repo.create(&shortened_url).await?)
    }

    async fn get_by_code(
        &self,
        code: impl Into<String> + Send,
    ) -> Result<ShortenedUrl, anyhow::Error> {
        let short_code = ShortCode::new(code)?;
        Ok(self.repo.get_by_code(&short_code).await?)
    }
}

impl<T: ShortenedUrlRepository> Service<T> {
    pub fn new(repo: T) -> Self {
        Self { repo }
    }
}
