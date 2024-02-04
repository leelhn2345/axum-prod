#![warn(clippy::pedantic)]

use axum_prod::configuration::get_configuration;
use axum_prod::startup::run;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("failed to read configuration");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .await
        .expect("error binding to port");
    run(listener, connection).await;
}
