use rsx::{Router, Server};
use pages::{home_page, about_page};

#[tokio::main]
async fn main() {
    let router = Router::new();
    
    router.route("/", || home_page());
    router.route("/about", || about_page());
    
    let server = Server::new(router);
    println!("Server running at http://localhost:3000");
    server.start().await;
}