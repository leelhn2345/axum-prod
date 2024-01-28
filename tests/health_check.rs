use tokio::net::TcpListener;

/// spawns server on a random port.
///
/// used for integration testing.
async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(axum_prod::run(listener));

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health"))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
/// *table-driven test*, also known as *parametrised test*.
///
/// instead of duplicated test logic several times, we simply run the same assertion
/// against a collection of known invalid bodies.
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%50gmail.com";

    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subsribe_retunrs_a_422_when_data_is_missing() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Bad Request when the payload was {error_message}"
        );
    }
}
