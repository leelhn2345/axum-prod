#![warn(clippy::pedantic)]

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("error binding to port");
    axum_prod::run(listener).await;
}
