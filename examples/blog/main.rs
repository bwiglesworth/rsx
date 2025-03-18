use rsx::components::{Component, ComponentBuilder};
use rsx::styles::Style;
use rsx::Router;
use rsx::Server;
use axum::response::Html;
use std::net::SocketAddr;

#[derive(Default, Clone)]
struct BlogState {
    posts: Vec<Post>
}

#[derive(Clone)]
struct Post {
    title: String,
    content: String,
    author: String,
}

#[derive(Clone)]
struct Blog {
    state: BlogState,
}

impl Component for Blog {
    type State = BlogState;

    fn render(&self) -> String {
        let mut style = Style::new();
        style.add_rule(".blog")
            .property("max-width", "800px")
            .property("margin", "0 auto")
            .property("padding", "20px");

        let posts = self.state.posts.iter()
            .map(|post| format!(
                "<article><h2>{}</h2><p>{}</p><small>By {}</small></article>",
                post.title, post.content, post.author
            ))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "<style>{}</style><div class='blog'><h1>My Blog</h1>{}</div>",
            style,
            posts
        )
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }
    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }
}

#[tokio::main]
async fn main() {
    let blog = Blog {
        state: BlogState {
            posts: vec![Post {
                title: "First Post".into(),
                content: "Welcome to RSX!".into(),
                author: "RSX Team".into(),
            }]
        }
    };

    let blog = ComponentBuilder::new(blog).build();
    let mut router = Router::new();
    router.route("/", move || Html(blog.render()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);
    
    let server = Server::new(router);
    server.start().await;
}