use crate::domain::url_shortener::models::{short_code::ShortCode, shortened_url::ShortenedUrl};

pub enum CreateUrlError {
    DBError,
}

pub enum GetByCodeError {
    DBError,
}

pub trait UrlRepository {
    fn create(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError>;
    fn get_by_code(&self, code: ShortCode) -> Result<ShortenedUrl, GetByCodeError>;
}
