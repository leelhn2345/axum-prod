use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::routes::{health_check, root, subscribe};

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
