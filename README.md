# RSX Web Framework

A modern Rust web framework built with Axum, featuring hot reload capabilities and a flexible styling system.

## Development Status

This project is under active development. While core features are functional, the API may change as we enhance and optimize the framework.

## Features

- Fast and efficient routing system
- Built-in hot reload functionality
- Customizable styling system
- Clean and intuitive API

## Example Usage

```rust
use axum::response::Html;
use rsx::{Router, Server};

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    
    router.route("/", || Html("<h1>Welcome to RSX!</h1>".to_string()));
    
    let server = Server::new(router);
    server.start().await;
}
```

## Testing

The framework includes comprehensive tests for:
- Server functionality
- Hot reload capabilities
- Styling system
- Component rendering

Run tests with:
```bash
cargo test
```

## Current Status

- ✅ Core routing system
- ✅ Hot reload implementation
- ✅ Style management
- ✅ Integration tests

## Roadmap

Upcoming features and improvements:

- [ ] Enhanced component system
- [ ] Optional client-side rendering capabilities
- [ ] State management
- [ ] Database integrations
- [ ] WebSocket support
- [ ] CLI tools for project scaffolding
- [ ] Advanced middleware system
- [ ] Performance optimizations
- [ ] Extended documentation and examples
- [ ] Hybrid rendering options (Server + Client)
- [ ] File-based routing system
- [ ] Image optimization and handling
- [ ] API routes
- [ ] Built-in CSS/Sass support
- [ ] Automatic code splitting
- [ ] Environment variable handling
- [ ] Static site generation (SSG)
- [ ] Incremental Static Regeneration (ISR)
- [ ] Dynamic imports
- [ ] SEO optimization tools
- [ ] Internationalization
- [ ] Analytics integration
- [ ] Edge functions
- [ ] Zero-config deployment- [ ] Hybrid rendering options (Server + Client)