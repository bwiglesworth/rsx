use rsx::{Router, Server};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    
    router.route("/", || Html(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>RSX Basic Website</title>
                <style>
                    body { font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
                    nav { margin: 20px 0; }
                    nav a { margin-right: 10px; }
                </style>
            </head>
            <body>
                <h1>Welcome to RSX Basic Website</h1>
                <nav>
                    <a href="/">Home</a>
                    <a href="/about">About</a>
                </nav>
                <p>This is your new RSX website!</p>
            </body>
        </html>
    "#.to_string()));

    let server = Server::new(router);
    println!("Server running at http://localhost:3000");
    server.start().await;
}
