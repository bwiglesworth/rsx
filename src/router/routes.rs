use axum::response::Html;use std::sync::Arc;

pub struct RouteConfig {
    path: String,
    handler: Arc<dyn Fn() -> Html<String> + Send + Sync>,
}

impl RouteConfig {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn get_handler(&self) -> &Arc<dyn Fn() -> Html<String> + Send + Sync> {
        &self.handler
    }
}