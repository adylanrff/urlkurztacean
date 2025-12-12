use std::{cell::RefCell, collections::HashMap};

use crate::domain::url_shortener::{
    models::{short_code::ShortCode, shortened_url::ShortenedUrl},
    ports::{CreateUrlError, GetByCodeError, ShortenedUrlRepository},
};

#[derive(Debug, Default)]
pub struct InMemoryStorage {
    store: RefCell<HashMap<ShortCode, ShortenedUrl>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            store: RefCell::new(HashMap::new()),
        }
    }

    fn add(&self, shortened_url: &ShortenedUrl) -> () {
        let mut hm = self.store.borrow_mut();
        hm.insert(shortened_url.short_code.clone(), shortened_url.clone());
    }

    fn get(&self, code: &ShortCode) -> Option<ShortenedUrl> {
        let hm = self.store.borrow();
        hm.get(code).cloned()
    }
}

impl ShortenedUrlRepository for InMemoryStorage {
    fn create(&self, shortened_url: &ShortenedUrl) -> Result<(), CreateUrlError> {
        if self.get(&shortened_url.short_code).is_some() {
            return Err(CreateUrlError::AlreadyExists);
        }

        self.add(shortened_url);
        Ok(())
    }

    fn get_by_code(&self, code: &ShortCode) -> Result<ShortenedUrl, GetByCodeError> {
        match self.get(code) {
            Some(shortened_url) => Ok(shortened_url),
            None => Err(GetByCodeError::NotFound),
        }
    }
}
