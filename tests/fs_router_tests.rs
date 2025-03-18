use rsx::router::FileRouter;
use std::fs;
use std::path::Path;

#[tokio::test]
async fn test_file_based_routing() {
    let pages_dir = Path::new("test_pages");
    fs::create_dir_all(pages_dir.join("api")).unwrap();
    
    fs::write(
        pages_dir.join("index.rs"),
        r#"Html("<h1>Home Page</h1>".to_string())"#,
    ).unwrap();
    
    let file_router = FileRouter::new(pages_dir);
    let routes = file_router.get_routes();
    
    // Print only the route paths
    println!("Found route paths: {:?}", routes.iter().map(|(path, _)| path).collect::<Vec<_>>());
    
    // Test actual handler execution
    let (_, handler) = &routes[0];
    let response = handler();
    println!("Handler response: {}", response.0);
    
    fs::remove_dir_all(pages_dir).unwrap();
    
    assert_eq!(response.0, "<h1>Home Page</h1>");
}