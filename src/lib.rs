#![warn(clippy::pedantic)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn root() -> Response {
    "hello world!".into_response()
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(_form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

/// runs the server. entry point of module.
///
/// # Panics
///
/// Panics if cannot start app
pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(listener, app).await.expect("cannot start app");
}
