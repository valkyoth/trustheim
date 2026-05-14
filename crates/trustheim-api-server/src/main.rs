#![forbid(unsafe_code)]

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = std::env::var("TRUSTHEIM_BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8787".to_string())
        .parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, trustheim_api_server::router()).await?;
    Ok(())
}
