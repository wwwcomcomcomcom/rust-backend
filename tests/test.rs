#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", "http://127.0.0.1:8000"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
fn spawn_app() {
    let server = rust_backend::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}