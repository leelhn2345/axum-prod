use std::net::TcpListener;

use axum::{http::StatusCode, routing::get, Error, Router};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Spawns an axum server
pub async fn run(addr: TcpListener) {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::from_tcp(addr)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
