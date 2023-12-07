#![warn(clippy::pedantic)]
use serde::Deserialize;
use tokio::net::TcpListener;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};

/// root function
async fn root() -> Response {
    // "hello world!".to_string()
    "hello world!".into_response()
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(_form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

/// Spawns an axum server
///
/// # Panics
///
/// Will panic if invalid TCP address is given.
pub async fn run(addr: TcpListener) {
    let app = Router::new()
        .route("/", get(root))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(addr, app).await.unwrap();
}
