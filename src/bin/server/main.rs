use std::env;

use sqlx::postgres::PgPoolOptions;
use urlkurztacean::{
    adapter::{
        inbound::http::HttpServer,
        outbound::storage::{in_memory::InMemoryStorage, postgres::PostgresStorage},
    },
    application::services::Service,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await?;

    let repo = PostgresStorage::new(pool);
    let service = Service::new(repo);

    let http_server = HttpServer::new(service).await;
    http_server.run().await
}
