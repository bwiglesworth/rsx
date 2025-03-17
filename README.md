# RSX - Rust Server Extensions Framework

A Next.js inspired web framework built in Rust, focusing on performance and developer experience.

## Completed Features
- HTTP Server with Axum integration
- Route configuration system
- Component-based architecture
- HTML template generation
- Asset compilation pipeline
- Hot reload development server

## Usage

```rust
use rsx::{init, Server, RouteConfig, AppTemplate};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let router = init();
    let mut server = Server::new(router);
    
    let template = AppTemplate::new("RSX App");
    server.add_route(RouteConfig::new("/", move || {
        Html(template.generate())
    }));

    server.start().await;
}
```

## Roadmap
1. Server-side rendering (SSR)
2. Static site generation (SSG)
3. File-based routing
4. API routes
5. Middleware support
6. Component lifecycle hooks
7. State management
8. CSS-in-Rust support

## Development

Build the project:
```bash
cargo build --release
```

Run the development server:
```bash
cargo run
```

## Contributing
RSX is under active development. Contributions are welcome!
