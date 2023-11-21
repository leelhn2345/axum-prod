use std::net::TcpListener;

use axum_prod::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("failed to bind to random port");
    run(listener).await;
}
