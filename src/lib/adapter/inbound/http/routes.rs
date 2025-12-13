use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    adapter::inbound::http::{
        handlers::{redirect_by_code, shorten_url},
        state::AppState,
    },
    application::ports::UrlShortenerService,
};

pub fn api_routes<S: UrlShortenerService>() -> axum::Router<AppState<S>> {
    Router::new()
        .route("/shorten", post(shorten_url::<S>))
        .route("/{code}", get(redirect_by_code::<S>))
}
