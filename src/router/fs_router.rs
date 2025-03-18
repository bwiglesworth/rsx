use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use axum::response::Html;

pub struct FileRouter {
    root_path: PathBuf,
    routes: Vec<RouteInfo>,
}

#[derive(Debug)]
struct RouteInfo {
    path: String,
    file_path: PathBuf,
    handler_type: HandlerType,
}

#[derive(Debug, Clone)]
pub enum HandlerType {
    Page,
    Api,
    Static,
}

impl FileRouter {
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        let mut router = Self {
            root_path: root_path.as_ref().to_path_buf(),
            routes: Vec::new(),
        };
        router.scan_routes();
        router
    }

    pub fn get_routes(&self) -> Vec<(String, Box<dyn Fn() -> Html<String> + Send + Sync>)> {
        self.routes
            .iter()
            .map(|route| {
                let content = fs::read_to_string(&route.file_path)
                    .unwrap_or_else(|_| "".to_string());
                let handler: Box<dyn Fn() -> Html<String> + Send + Sync> = 
                    Box::new(move || Self::eval(&content));
                (route.path.clone(), handler)
            })
            .collect()
    }

    fn eval(content: &str) -> Html<String> {
        // Remove the Html wrapper and .to_string() from the content
        let html_content = content
            .trim_start_matches("Html(\"")
            .trim_end_matches("\".to_string())")
            .to_string();
        Html(html_content)
    }

    fn scan_routes(&mut self) {
        for entry in WalkDir::new(&self.root_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if let Some(route) = self.entry_to_route(entry) {
                self.routes.push(route);
            }
        }
    }

    fn entry_to_route(&self, entry: walkdir::DirEntry) -> Option<RouteInfo> {
        let file_path = entry.path().to_path_buf();
        if !file_path.is_file() {
            return None;
        }

        let path = self.file_path_to_route(&file_path)?;
        let handler_type = self.determine_handler_type(&file_path);

        Some(RouteInfo {
            path,
            file_path,
            handler_type,
        })
    }

    fn file_path_to_route(&self, file_path: &Path) -> Option<String> {
        file_path
            .strip_prefix(&self.root_path)
            .ok()
            .map(|relative| {
                let mut path = relative
                    .with_extension("")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                
                if path == "index" {
                    path = String::from("/");
                } else if !path.starts_with('/') {
                    path = format!("/{}", path);
                }
                path
            })
    }

    fn determine_handler_type(&self, path: &Path) -> HandlerType {
        match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => HandlerType::Page,
            Some("json") => HandlerType::Api,
            _ => HandlerType::Static,
        }
    }

    pub fn get_route_content(&self, path: &str) -> Option<String> {
        self.routes
            .iter()
            .find(|route| route.path == path)
            .and_then(|route| fs::read_to_string(&route.file_path).ok())
    }
}