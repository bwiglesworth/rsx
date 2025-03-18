use rsx::{Router, Server};
use axum::response::Html;
use std::fs;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::test]
async fn test_hot_reload() {
    let test_file = "test_component.html";
    fs::write(test_file, "<div>Initial Content</div>").unwrap();

    let mut router = Router::new();
    router.route("/hot", move || {
        let content = fs::read_to_string(test_file).unwrap();
        Html(content)
    });

    let server = Server::new(router);
    
    tokio::spawn(async move {
        server.start().await;
    });

    sleep(Duration::from_millis(100)).await;

    let response = reqwest::get("http://localhost:3000/hot").await.unwrap();
    assert_eq!(response.text().await.unwrap(), "<div>Initial Content</div>");

    fs::write(test_file, "<div>Updated Content</div>").unwrap();
    sleep(Duration::from_millis(100)).await;

    let response = reqwest::get("http://localhost:3000/hot").await.unwrap();
    assert_eq!(response.text().await.unwrap(), "<div>Updated Content</div>");

    fs::remove_file(test_file).unwrap();
}