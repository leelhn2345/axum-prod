#![warn(clippy::pedantic)]

use axum_prod::configuration::get_configuration;
use axum_prod::startup::run;
use axum_prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("axumProd".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");
    // no longer async, given that we don't actually try to connect!
    let connection = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("failed to connect to postgres.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)
        .await
        .expect("error binding to port");
    run(listener, connection).await;
}
