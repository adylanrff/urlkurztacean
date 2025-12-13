use anyhow::Ok;
use axum::routing::get;
use tokio::net;

use crate::{
    adapter::inbound::http::{handlers::hello_world, routes::api_routes, state::AppState},
    application::ports::UrlShortenerService,
};

mod handlers;
mod routes;
mod state;

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new(service: impl UrlShortenerService) -> Self {
        let app_state = AppState::new(service);
        let router = axum::Router::new()
            .route("/", get(hello_world))
            .nest("/api", api_routes())
            .with_state(app_state);

        let listener = net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        Self {
            router,
            listener,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        axum::serve(self.listener, self.router).await?;
        Ok(())
    }
}
