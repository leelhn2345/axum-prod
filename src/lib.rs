#![warn(clippy::pedantic)]
use tokio::net::TcpListener;

use axum::{http::StatusCode, routing::get, Router};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Spawns an axum server
///
/// # Panics
///
/// Will panic if invalid TCP address is given.
pub async fn run(addr: TcpListener) {
    let app = Router::new().route("/health_check", get(health_check).post(health_check));
    axum::serve(addr, app).await.unwrap();
}
