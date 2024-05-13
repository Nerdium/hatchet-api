//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();

    let server = hatchet::run(listener).expect("Failed to bind listener.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
