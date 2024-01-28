use axum::{http::StatusCode, Form};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}
