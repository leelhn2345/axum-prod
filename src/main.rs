#![warn(clippy::pedantic)]

use axum_prod::configuration::get_configuration;
use axum_prod::startup::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .await
        .expect("error binding to port");
    run(listener).await;
}
