# RSX - Server-Side React-like Components in Rust

![Status](https://img.shields.io/badge/Status-Under%20Development-blue)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen)
![Version](https://img.shields.io/badge/Version-0.6.0-orange)
![Release Date](https://img.shields.io/badge/Production%20Release-Jan%201%202026-yellow)

RSX is a lightweight framework for building server-side rendered React-like components in Rust, powered by Axum. The framework is actively progressing toward a production release targeted for August 14, 2024.

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
## ROADMAP to Production

### Phase 1: Core Framework (Q1 2024)
- Complete HTML sanitization system
- Implement proper error handling and logging
- Add middleware support for request/response processing
- Build asset bundling and optimization pipeline
- Create production-ready server configuration

### Phase 2: Developer Experience (Q2 2024)
- Hot reload development server
- CLI tool for project scaffolding
- Debug tooling with component inspection
- Development mode with detailed error reporting
- Documentation site with interactive examples

### Phase 3: Performance & Scale (Q3 2024)
- Component caching system
- Static site generation capabilities
- Server-side data fetching layer
- Database integration patterns
- Load balancing support

### Phase 4: Production Features (Q4 2024)
- Authentication system
- Session management
- Form handling with validation
- File upload processing
- Production monitoring tools

### Phase 5: Advanced Features (Q1 2025)
- Component hydration for client-side interactivity
- TypeScript/JavaScript integration
- WebSocket support
- Server-sent events
- GraphQL integration

Each phase builds upon the previous one to create a complete, production-ready web framework that maintains the simplicity and performance of RSX while providing all necessary features for modern web applications.

## License
MIT Licensed. See LICENSE for details.