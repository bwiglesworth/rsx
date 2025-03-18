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

## Creating a Website with RSX

### Project Structure
```
my-rsx-website/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── components/
│       └── navbar.rs
├── pages/
│   ├── index.rs
│   ├── about.rs
│   └── blog.rs
└── styles/
    └── global.rs
```

### 1. Create a Navigation Component in `src/components/navbar.rs`:

```rust
use rsx::components::Component;
use rsx::styles::Style;

struct NavBar;

impl Component for NavBar {
    fn render(&self) -> String {
        let mut style = Style::new();
        style.add_rule("nav")
            .property("background", "#333")
            .property("padding", "1rem")
            .property("color", "white");

        format!(
            "<style>{}</style>
            <nav>
                <a href='/'>Home</a>
                <a href='/about'>About</a>
                <a href='/blog'>Blog</a>
            </nav>",
            style
        )
    }
}
```

### 2. Set Up Your Website in `src/main.rs`:

```rust
use rsx::{Router, Server};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let navbar = NavBar;
    let home_page = HomePage::new();
    
    let mut router = Router::new();
    router.route("/", move || {
        let content = format!(
            "<div class='app'>
                {}
                {}
            </div>",
            navbar.render(),
            home_page.render()
        );
        Html(content)
    });

    let server = Server::new(router);
    println!("Server running at http://localhost:3000");
    server.start().await;
}
```

RSX will automatically handle routing based on your pages directory structure,MIT