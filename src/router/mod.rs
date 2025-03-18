mod routes;
pub use routes::RouteConfig;
use axum::{Router as AxumRouter, routing::get, response::Html};

pub struct Router {
    inner: AxumRouter
}

impl Router {
    pub fn new() -> Self {
        Router {
            inner: AxumRouter::new()
        }
    }

    pub fn route<H>(&mut self, path: &str, handler: H) -> &mut Self 
    where 
        H: Fn() -> Html<String> + Clone + Send + Sync + 'static,
    {
        let handler = move || async move { handler() };
        self.inner = self.inner.clone().route(path, get(handler));
        self
    }

    pub fn build(self) -> AxumRouter {
        self.inner
    }
}