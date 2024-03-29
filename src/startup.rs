use std::time::Duration;

use axum::{
    body::Body,
    http::Request,
    response::Response,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Span;

use crate::{
    routes::{health_check, root, subscribe},
    send_email::EmailClient,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// runs the server. entry point of module.
///
/// # Panics
///
/// Panics if cannot start app.
pub async fn run(listener: TcpListener, db_pool: PgPool, email_client: EmailClient) {
    // axum middleware trace layer
    let trace_layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::info_span!(
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id),
                    latency = tracing::field::Empty,
                    status_code = tracing::field::Empty,
                )
            })
            .on_response(
                |response: &Response<Body>, latency: Duration, span: &Span| {
                    span.record("status_code", &tracing::field::display(response.status()));
                    span.record("latency", &tracing::field::debug(latency));
                },
            ),
    );
    let app = Router::new()
        .route("/", get(root))
        .route("/subscriptions", post(subscribe))
        .with_state(db_pool)
        .with_state(email_client)
        .layer(trace_layer)
        // put `/health_check` after trace layer to reduce noise in logs.
        .route("/health_check", get(health_check));

    if let Ok(addr) = listener.local_addr() {
        tracing::info!("starting service on {}", addr);
    }

    axum::serve(listener, app).await.expect("cannot start app");
}
