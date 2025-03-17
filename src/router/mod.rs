mod routes;
pub use routes::RouteConfig;

pub struct Router {
    routes: Vec<Route>,
}

pub struct Route {
    path: String,
    handler: Box<dyn RouteHandler>
}

impl Route {
    pub fn execute(&self) -> String {
        self.handler.handle()
    }

    pub fn new(path: String, handler: Box<dyn RouteHandler>) -> Self {
        Route { path, handler }
    }
}

pub trait RouteHandler {
    fn handle(&self) -> String;
}
impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
        }
    }

    pub fn execute_route(&self, path: &str) -> Option<String> {
        self.routes.iter()
            .find(|route| route.path == path)
            .map(|route| route.execute())
    }

    pub fn add_route(&mut self, path: String, handler: Box<dyn RouteHandler>) {
        self.routes.push(Route { path, handler });
    }
}
