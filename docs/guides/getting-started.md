# Getting Started with RSX

Add RSX to your project:
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
        "<h1>Hello, RSX!</h1>".to_string()
    }
}
```
```

```markdown:/docs/guides/component-creation.md
# Component Creation

Components in RSX follow a simple pattern:

1. Define your component structure
2. Implement the Component trait
3. Add state management if needed
4. Style your component

Example with state and styling:
```rust
use rsx::components::Component;
use rsx::styles::Style;

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
        let mut style = Style::new();
        style.add_rule(".counter")
            .property("padding", "1rem")
            .property("text-align", "center");

        format!(
            "<div class='counter'>
                <style>{}</style>
                <h2>Count: {}</h2>
                <button onclick='increment()'>+1</button>
            </div>",
            style,
            self.state.count
        )
    }
}
```
```

```markdown:/docs/guides/routing.md
# Routing in RSX

RSX uses file-based routing:

```
pages/
  index.rs      -> "/"
  about.rs      -> "/about"
  blog/
    index.rs    -> "/blog"
    [slug].rs   -> "/blog/:slug"
```

Dynamic routes example:
```rust
// pages/blog/[slug].rs
impl Component for BlogPost {
    fn render(&self) -> String {
        format!("<article>{}</article>", self.content)
    }
}
```
