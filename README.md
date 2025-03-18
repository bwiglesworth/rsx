# RSX - Rust Server Components

A lightweight framework for building server-side rendered React-like components in Rust.

## Features

- Server-side component rendering
- File-based routing
- Hot module reloading
- Lifecycle management
- Built-in styling system
- Async component support

## Getting Started

Add RSX to your Cargo.toml:

```toml
[dependencies]
rsx = "0.1.0"
```

Create your first component:

```rust
use rsx::components::Component;

struct HelloWorld;

impl Component for HelloWorld {
    fn render(&self) -> String {
        "<h1>Hello, World!</h1>".to_string()
    }
}
```

## Testing

Run the test suite:
```bash
cargo test
```

## Examples

Check out the examples directory for more usage patterns and component implementations.

## License

MIT