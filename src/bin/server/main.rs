use urlkurztacean::{
    adapter::{inbound::http::HttpServer, outbound::storage::in_memory::InMemoryStorage},
    application::services::Service,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let repo = InMemoryStorage::new();
    let service = Service::new(repo);

    let http_server = HttpServer::new(service).await;
    http_server.run().await
}
