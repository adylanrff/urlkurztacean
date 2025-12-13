use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

use crate::domain::url_shortener::{
    models::{short_code::ShortCode, shortened_url::ShortenedUrl},
    ports::{CreateUrlError, GetByCodeError, ShortenedUrlRepository},
};

#[derive(Debug, Default)]
pub struct InMemoryStorage {
    store: Arc<RwLock<HashMap<ShortCode, ShortenedUrl>>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn add(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError> {
        let mut hm = self.store.write().map_err(|_| CreateUrlError::LockError)?;
        if hm.contains_key(&shortened_url.short_code) {
            return Err(CreateUrlError::AlreadyExists);
        }

        hm.insert(shortened_url.short_code.clone(), shortened_url.clone());
        Ok(())
    }

    fn get(&self, code: &ShortCode) -> Option<ShortenedUrl> {
        let hm = self.store.read().unwrap();
        hm.get(code).cloned()
    }
}

#[async_trait]
impl ShortenedUrlRepository for InMemoryStorage {
    async fn create(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError> {
        self.add(shortened_url)?;
        Ok(())
    }

    async fn get_by_code(&self, code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError> {
        match self.get(code) {
            Some(shortened_url) => Ok(shortened_url),
            None => Err(GetByCodeError::NotFound),
        }
    }
}
