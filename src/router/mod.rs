mod fs_router;
pub use fs_router::FileRouter;
use axum::{Router as AxumRouter, routing::get, response::Html};
use std::path::Path;
pub struct Router {
    inner: AxumRouter,
    file_router: Option<FileRouter>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            inner: AxumRouter::new(),
            file_router: None,
        }
    }

    pub fn with_pages<P: AsRef<Path>>(pages_dir: P) -> Self {
        let mut router = Self::new();
        router.file_router = Some(FileRouter::new(pages_dir));
        
        if let Some(file_router) = &router.file_router {
            for (path, content) in file_router.get_routes() {
                let content_str = content().0;
                let handler = move || Html(content_str.clone());
                router.route(&path, handler);
            }
        }
        
        router
    }
    pub fn route<H>(&mut self, path: &str, handler: H) -> &mut Self 
    where 
        H: Fn() -> Html<String> + Clone + Send + Sync + 'static
    {
        let async_handler = move || async move { handler() };
        self.inner = self.inner.clone().route(path, get(async_handler));
        self
    }

    pub fn build(self) -> AxumRouter {
        self.inner
    }
}