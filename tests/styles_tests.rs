use rsx::{Router, Server};
use axum::response::Html;

#[tokio::test]
async fn test_styled_page() {
    let mut router = Router::new();
    
    router.route("/styles", || Html(r#"
        <div class="container">
            <h1 class="title">RSX Styles Test Page</h1>
            <div class="card">
                <h2>Primary Colors</h2>
                <div class="color-box primary">Primary</div>
                <div class="color-box secondary">Secondary</div>
            </div>
            <div class="card">
                <h2>Typography</h2>
                <p class="text-large">Large Text</p>
                <p class="text-medium">Medium Text</p>
                <p class="text-small">Small Text</p>
            </div>
        </div>
    "#.to_string()));

    let server = Server::new(router);
    
    tokio::spawn(async move {
        server.start().await;
    });
    
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    let response = reqwest::get("http://localhost:3000/styles").await.unwrap();
    assert_eq!(response.status(), 200);
}
