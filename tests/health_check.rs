use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check() {
    let listener = TcpListener::bind("localhost:0").expect("Failed to find an open port.");
    let port = listener
        .local_addr()
        .expect("Unable to determine local port")
        .port();
    let url = format!("http://localhost:{port}/health_check");

    let _ = tokio::spawn(run(listener));

    let response = reqwest::get(url).await.expect("Failed to run health check");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
