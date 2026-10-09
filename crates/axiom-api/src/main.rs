use axiom_api::app;
use axiom_storage::MemoryRepository;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let repo = MemoryRepository::new();
    let app = app(repo);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse::<u16>().unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("AXIOM Mathematics API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
