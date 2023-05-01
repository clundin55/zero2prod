use anyhow::Result;
use std::net::TcpListener;

use zero2prod::startup::run;

async fn spawn_app() -> Result<String> {
    let listener = TcpListener::bind("localhost:0").expect("Failed to find an open port.");
    let port = listener
        .local_addr()
        .expect("Unable to determine local port")
        .port();
    let url = format!("http://localhost:{port}");
    let _ = tokio::spawn(run(listener));
    Ok(url)
}

#[tokio::test]
async fn subscribe_200_valid_form_data() {
    let url = spawn_app().await.unwrap();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{url}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_400_when_data_missing() {
    let url = spawn_app().await.unwrap();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{url}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
