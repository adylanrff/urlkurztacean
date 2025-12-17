use axum::{
    Json,
    extract::{State, path::Path},
    http::StatusCode,
    response::Redirect,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{adapter::inbound::http::state::AppState, application::ports::UrlShortenerService};

pub async fn hello_world() -> &'static str {
    "Hello world!"
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortenUrlPayload {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortenUrlResponse {
    shortened_url: String,
}

#[instrument(skip(state))]
pub async fn shorten_url<S: UrlShortenerService>(
    State(state): State<AppState<S>>,
    Json(payload): Json<ShortenUrlPayload>,
) -> Result<Json<ShortenUrlResponse>, (StatusCode, String)> {
    let res = state
        .service
        .shorten_url(payload.url)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let shortened_url = format!("/api/{}", res.short_code);

    Ok(Json(ShortenUrlResponse { shortened_url }))
}

#[instrument(skip(state))]
pub async fn redirect_by_code<S: UrlShortenerService>(
    State(state): State<AppState<S>>,
    Path(code): Path<String>,
) -> Result<Redirect, (StatusCode, String)> {
    let res = state
        .service
        .get_by_code(code)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    let original_url = res.original_url.to_string();
    Ok(Redirect::permanent(&original_url))
}
