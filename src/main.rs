use axum::response::Html;
use rsx::{Router, Server};

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    
    router.route("/", || Html("<h1>Welcome to RSX!</h1>".to_string()));
    
    let server = Server::new(router);
    server.start().await;
}