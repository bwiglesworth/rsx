# RSX Web Framework

A modern Rust web framework built with Axum, featuring hot reload capabilities and a flexible styling system.

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
