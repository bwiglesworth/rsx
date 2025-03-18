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

#[derive(Default)]
struct CounterState {
    count: i32
}

struct Counter {
    state: CounterState
}

impl Component for Counter {
    type State = CounterState;
    
    fn render(&self) -> String {
        format!("<div>Count: {}</div>", self.state.count)
    }
}
```

## File-Based Routing

RSX uses a file-based routing system similar to Next.js:

```
pages/
  index.rs      -> "/"
  about.rs      -> "/about"
  blog/
    index.rs    -> "/blog"
    [slug].rs   -> "/blog/:slug"
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