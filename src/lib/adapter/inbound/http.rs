use std::time::Duration;

use axum::{http, routing::get};
use tokio::{net, signal};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrlc handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate handler")
            .recv()
            .await
    };

    tokio::select! {
        _ = ctrl_c => tracing::info!("Ctrl+C signal received. shutting down"),
        _ = terminate =>tracing::info!("SIGTERM signal received. shutting down"),
    }
}

impl HttpServer {
    pub async fn new(service: impl UrlShortenerService) -> Self {
        let app_state = AppState::new(service);
        let router = axum::Router::new()
            .route("/", get(hello_world))
            .nest("/api", api_routes())
            .layer(TraceLayer::new_for_http())
            .layer(TimeoutLayer::with_status_code(
                http::StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(30),
            ))
            .with_state(app_state);

        let listener = net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        Self { router, listener }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("running server at 0.0.0.0:3000");

        axum::serve(self.listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        tracing::info!("server shut down");
        Ok(())
    }
}
