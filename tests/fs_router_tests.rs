use rsx::Router;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_file_based_routing() {
    // Create test directory structure
    let pages_dir = Path::new("test_pages");
    fs::create_dir_all(pages_dir.join("api")).unwrap();
    
    fs::write(
        pages_dir.join("index.rs"),
        r#"Html("<h1>Home Page</h1>".to_string())"#,
    ).unwrap();
    
    fs::write(
        pages_dir.join("about.rs"),
        r#"Html("<h1>About Page</h1>".to_string())"#,
    ).unwrap();

    // Initialize router with pages directory
    let router = Router::with_pages(pages_dir);
    
    // Start server
    let server = rsx::Server::new(router);
    tokio::spawn(async move {
        server.start().await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Test routes
    let home = reqwest::get("http://localhost:3000").await.unwrap();
    assert_eq!(home.text().await.unwrap(), "<h1>Home Page</h1>");

    let about = reqwest::get("http://localhost:3000/about").await.unwrap();
    assert_eq!(about.text().await.unwrap(), "<h1>About Page</h1>");

    // Cleanup
    fs::remove_dir_all(pages_dir).unwrap();
}
