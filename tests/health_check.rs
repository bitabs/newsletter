use std::net::TcpListener;

use newsletter::{config::get_config, startup::run};
use sqlx::{Connection, PgConnection};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http:127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = &format!("{}/health_check", &app_addr);

    // Act
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange
    let app_addr = spawn_app();
    let config = get_config().expect("Failed to read config");
    let connection_str = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_str)
        .await
        .expect("Failed to connect to postgres");
    let client = reqwest::Client::new();
    let endpoint = format!("{}/subscription", &app_addr);

    // Act
    let body = "name=john%20doe&email=john.doe%40gmail.com";
    let resp = client
        .post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, resp.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.name, "john doe");
    assert_eq!(saved.email, "john.doe@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange
    let app_addr = spawn_app();
    let client = reqwest::Client::new();
    let endpoint = format!("{}/subscription", &app_addr);

    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john.doe%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        // Act
        let resp = client
            .post(&endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            resp.status().as_u16(),
            "The API did not fail with 400 Bad request when the payload was {}",
            error_msg
        );
    }
}
