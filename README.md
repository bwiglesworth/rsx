# RSX - Server-Side React-like Components in Rust

![Status](https://img.shields.io/badge/Status-Under%20Development-blue)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)


RSX is a lightweight framework for building server-side rendered React-like components in Rust, powered by Axum.
## Features

- **File-System Based Routing**: Automatically generate routes from your directory structure
- **Component System**: Build reusable UI components with state management
- **Dynamic Routes**: Support for dynamic parameters in URLs
- **Fast & Lightweight**: Built on top of Axum for high performance

## Quick Start

Add RSX to your project:
```toml
[dependencies]
rsx = "1.0.0"
```

Create your first component:
```rust
use rsx::Component;

#[derive(Default, Clone)]
struct BlogState {
     posts: Vec<Post>
}

struct Blog {
     state: BlogState,
}

impl Component for Blog {
     type State = BlogState;

     fn render(&self) -> String {
         let mut style = Style::new();
         style.add_rule(".blog")
             .property("max-width", "800px")
             .property("margin", "0 auto");

         let posts = self.state.posts.iter()
             .map(|post| format!(
                 "<article><h2>{}</h2><p>{}</p></article>",
                 post.title, post.content
             ))
             .collect::<Vec<_>>()
             .join("\n");

         format!("<div class='blog'>{}{}</div>", style, posts)
     }
}
```

## File-System Based Routing

RSX uses your directory structure to create routes:

```
pages/
   index.rs      -> "/"
   about.rs      -> "/about"
   blog/
     index.rs    -> "/blog"
     [slug].rs   -> "/blog/:slug"
```

## Current Implementation Status

### Completed Features
- File system router implementation with static route handling
- Basic component system with state management
- Blog example implementation
- Integration with Axum web framework
- Dynamic route parameter support ([slug] style routes)

### Testing Coverage
- File system router tests validating route generation
- Route handler execution verification
- Path matching validation
- Basic component rendering tests through the blog example
## Contributing

RSX is open to contributions. Feel free to submit issues and pull requests.

## License

MIT Licensed. See LICENSE for details.