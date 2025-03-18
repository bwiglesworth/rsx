pub mod router;
pub mod server;
pub mod components;
pub mod builder;
pub mod templates;
pub mod styles;

// Re-exports for convenient usage
pub use router::Router;
pub use router::RouteConfig;
pub use server::Server;
pub use components::{Component, Props};
pub use builder::Builder;
pub use templates::AppTemplate;
pub use styles::{Style, StyleRule};

// Version and framework info
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FRAMEWORK_NAME: &str = "RSX";

// Initialize framework
pub fn init() -> Router {
    Router::new()
}