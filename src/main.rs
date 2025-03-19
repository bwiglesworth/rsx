use axum::response::Html;
use rsx::{Router, Server};

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    
    // Add some basic routes
    router.route("/", || Html("<h1>Welcome to RSX!</h1>".to_string()));
    
    router.route("/about", || Html(r#"
        <div>
            <h1>About Page</h1>
            <p>This is a basic RSX server example.</p>
        </div>
    "#.to_string()));

    let server = Server::new(router);
    println!("Server running at http://localhost:3000");
    server.start().await;
}