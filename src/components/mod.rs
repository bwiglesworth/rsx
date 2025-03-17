use std::fmt;

pub trait Component: fmt::Display {
    fn render(&self) -> String;
    fn get_props(&self) -> Props;
}

#[derive(Default)]
pub struct Props {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
}

pub struct Element {
    tag: String,
    props: Props,
    children: Vec<Box<dyn Component>>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Element {
            tag: tag.to_string(),
            props: Props::default(),
            children: Vec::new(),
        }
    }

    pub fn render(&self) -> String {
        format!("<{} {}>{}</{}>", 
            self.tag,
            self.props.class_name.as_deref().unwrap_or(""),
            self.children.iter().map(|c| c.render()).collect::<String>(),
            self.tag
        )
    }

    pub fn add_child(&mut self, child: Box<dyn Component>) {
        self.children.push(child);
    }
}
