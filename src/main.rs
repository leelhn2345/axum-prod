#![warn(clippy::pedantic)]

use axum_prod::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to random port");

    run(listener).await;
}
