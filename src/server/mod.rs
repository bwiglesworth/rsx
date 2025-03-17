use axum::{Router as AxumRouter};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::router::{Router, RouteConfig};

pub struct Server {
    router: Router,
    port: u16,
    routes: Vec<RouteConfig>,
}
impl Server {
    pub fn new(router: Router) -> Self {
        Server {
            router,
            port: 3000,
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: RouteConfig) {
        self.routes.push(route);
    }

    pub fn get_router(&self) -> &Router {
        &self.router
    }

    pub async fn start(&self) {
        let mut app = AxumRouter::new();
        
        for route in &self.routes {
            let (_path, route_handler) = route.into_route();            app = app.merge(route_handler);
        }

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        println!("Server running on http://{}", addr);
        
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }}