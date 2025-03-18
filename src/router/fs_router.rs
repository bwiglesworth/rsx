use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use axum::response::Html;

pub struct FileRouter {
    pages_dir: PathBuf,
    routes: Vec<RouteInfo>,
}

struct RouteInfo {
    path: String,
    file_path: PathBuf,
    content: String,
}

impl FileRouter {
    pub fn new<P: AsRef<Path>>(pages_dir: P) -> Self {
        let mut router = FileRouter {
            pages_dir: pages_dir.as_ref().to_path_buf(),
            routes: Vec::new(),
        };
        router.scan_routes();
        router
    }

    pub fn scan_routes(&mut self) {
        for entry in WalkDir::new(&self.pages_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let route_path = self.file_path_to_route(entry.path());
            let content = std::fs::read_to_string(entry.path()).unwrap();
            let route_info = RouteInfo {
                path: route_path,
                file_path: entry.path().to_path_buf(),
                content,
            };
            self.routes.push(route_info);
        }
    }
    pub fn get_routes(&self) -> Vec<(String, Html<String>)> {
        self.routes.iter()
            .map(|route| {
                let content = route.content
                    .trim_start_matches("Html(\"")
                    .trim_end_matches("\".to_string())")
                    .to_string();
                (route.path.clone(), Html(content))
            })
            .collect()
    }

    fn file_path_to_route(&self, file_path: &Path) -> String {
        let relative_path = file_path.strip_prefix(&self.pages_dir).unwrap();
        let mut route = relative_path
            .with_extension("")
            .to_string_lossy()
            .replace(std::path::MAIN_SEPARATOR, "/");
            
        if !route.starts_with('/') {
            route = format!("/{}", route);
        }
        
        if route.ends_with("/index") {
            route = route.trim_end_matches("/index").to_string();
            if route.is_empty() {
                route = "/".to_string();
            }
        }
        route
    }}