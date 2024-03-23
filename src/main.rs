#![warn(clippy::pedantic)]

use axum_prod::configuration::get_configuration;
use axum_prod::send_email::EmailClient;
use axum_prod::startup::run;
use axum_prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("axumProd".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");
    // no longer async, given that we don't actually try to connect!
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)
        .await
        .expect("error binding to port");
    run(listener, connection_pool, email_client).await;
}
