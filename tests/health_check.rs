#[tokio::test]
async fn health_check_success() {
    spawn_app();
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}