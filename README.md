# RSX - Rust Server Components

A modern web framework for building server-side rendered applications in Rust.

## Completed Features

- File-based routing
- Hot reload during development
- Component system with nested children support
- Built-in styling system
- Server-side rendering

## Component Example

```rust
use rsx::components::{Component, ComponentBuilder};

// Create a card component
let card = Card {
    title: "Welcome".to_string(),
    content: "This is a RSX card component".to_string(),
    children: Vec::new(),
};

let card = ComponentBuilder::new(card)
    .build();

// Render the component
card.render();
```

Ready to add more examples or enhance the documentation further!

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