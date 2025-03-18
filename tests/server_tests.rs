use axum::response::Html;
use rsx::{Router, Server};

#[tokio::test]
async fn test_basic_route() {
    let mut router = Router::new();
    router.route("/", || Html("<h1>Welcome to RSX!</h1>".to_string()));
    
    let server = Server::new(router);
    
    // Start server in background
    tokio::spawn(async move {
        server.start().await;
    });
    
    // Give server time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    // Test the endpoint
    let response = reqwest::get("http://localhost:3000").await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "<h1>Welcome to RSX!</h1>");
}
