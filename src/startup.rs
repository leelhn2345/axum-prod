use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;

/// Spawns an axum server
///
/// # Panics
///
/// Will panic if invalid TCP address is given.
pub async fn run(addr: TcpListener, connection: PgPool) {
    let app = Router::new()
        .route("/", get(|| async { "hello world".to_string() }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection);

    axum::serve(addr, app).await.unwrap();
}
