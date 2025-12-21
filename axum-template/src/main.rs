use anyhow::Context;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

use crate::error_adapter::AppError;

mod error_adapter;
mod graceful_shutdown;
mod routes;

async fn run_server() -> Result<(), AppError> {
    let host = std::env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port: u16 = std::env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
        .context("invalid port")?;
    let tcp_listener = TcpListener::bind((host.as_str(), port))
        .await
        .context("could not set up TCP listener")?;

    tracing::debug!(host = host, port = port, "set up TCP listener");

    let router = routes::get_router();
    Ok(axum::serve(tcp_listener, router)
        .with_graceful_shutdown(graceful_shutdown::graceful_shutdown())
        .await?)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    if let Err(e) = run_server().await {
        tracing::error!("{e}");
    }
}
