use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[tracing::instrument(name = "health check")]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "root")]
pub async fn root() -> Response {
    "hello world!".into_response()
}
