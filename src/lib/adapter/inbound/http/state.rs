use crate::application::ports::UrlShortenerService;

#[derive(Debug, Clone)]
pub struct AppState<S: UrlShortenerService> {
    pub service: S,
}

impl<S: UrlShortenerService> AppState<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}
