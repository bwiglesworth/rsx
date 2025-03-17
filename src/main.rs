use rsx::{init, Server, RouteConfig, AppTemplate};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let router = init();
    let mut server = Server::new(router);
    
    // Add a simple route
    let template = AppTemplate::new("RSX App");
    server.add_route(RouteConfig::new("/", move || {
        Html(template.generate())
    }));

    server.start().await;
}