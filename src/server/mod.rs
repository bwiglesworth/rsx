use crate::Router;
use std::net::SocketAddr;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Server { router }
    }

    pub async fn start(self) {
        let app = self.router.build();
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("Server running on http://{}", addr);
        axum::serve(listener, app).await.unwrap();
    }
}