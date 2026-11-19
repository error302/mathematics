use axiom_api::local_demo_app;
use axiom_storage::MemoryRepository;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    if std::env::var("AXIOM_LOCAL_DEMO").as_deref() != Ok("1") {
        return Err("Account service is not implemented. Set AXIOM_LOCAL_DEMO=1 only for a synthetic loopback demonstration.".into());
    }
    let repo = MemoryRepository::new();
    let app = local_demo_app(repo);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into()).parse::<u16>().unwrap_or(8080);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("AXIOM Mathematics API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
