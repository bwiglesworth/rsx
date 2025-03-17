use axum::{
    response::Html,
    routing::get,
    Router as AxumRouter,
};
use std::sync::Arc;

pub struct RouteConfig {
    path: String,
    handler: Arc<dyn Fn() -> Html<String> + Send + Sync>,
}

impl RouteConfig {
    pub fn new<F>(path: &str, handler: F) -> Self 
    where 
        F: Fn() -> Html<String> + Send + Sync + 'static,
    {
        RouteConfig {
            path: path.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub fn into_route(&self) -> (String, AxumRouter) {
        let handler = self.handler.clone();
        let router = AxumRouter::new()
            .route("/", get(move || async move { (handler)() }));
        (self.path.clone(), router)
    }
}