use std::env;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use urlkurztacean::{
    adapter::{inbound::http::HttpServer, outbound::storage::postgres::PostgresStorage},
    application::services::Service,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    let repo = PostgresStorage::new(pool);
    let service = Service::new(repo);

    let http_server = HttpServer::new(service).await;
    http_server.run().await
}
