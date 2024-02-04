use axum::{extract::State, http::StatusCode, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(connection): State<PgPool>, Form(form): Form<FormData>) -> StatusCode {
    match sqlx::query(
        r"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        ",
    )
    .bind(Uuid::new_v4())
    .bind(form.email)
    .bind(form.name)
    .bind(Utc::now())
    .execute(&connection)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("failed to execute query: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
