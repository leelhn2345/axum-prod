use axum::{extract::State, http::StatusCode, Form};
use serde::Deserialize;
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    State(_connection): State<PgPool>,
    Form(_form): Form<FormData>,
) -> StatusCode {
    StatusCode::OK
}
