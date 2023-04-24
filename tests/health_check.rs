use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check() {
    let listener = TcpListener::bind("localhost:0").expect("Failed to find an open port.");
    let _ = tokio::spawn(run(listener));

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:10000/health_check")
        .send()
        .await
        .expect("Failed to run health check");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
