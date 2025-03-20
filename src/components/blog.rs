#[derive(Clone)]
pub struct Post {
    title: String,
    content: String,
    author: String,
}

#[derive(Default, Clone)]
pub struct BlogState {
    posts: Vec<Post>
}

pub struct Blog {
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
