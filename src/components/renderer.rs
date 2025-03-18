use proc_macro2::TokenStream;
use quote::quote;

pub struct ComponentRenderer<T: Component> {
    component: T,
}

impl<T: Component> ComponentRenderer<T> {
    pub fn new(component: T) -> Self {
        Self { component }
    }

    pub fn render(&self) -> String {
        let props = self.component.get_props();
        let class_attr = props.class_name.as_ref()
            .map(|c| format!(" class=\"{}\"", c))
            .unwrap_or_default();
            
        let children = self.component.children()
            .iter()
            .map(|child| child.render())
            .collect::<Vec<_>>()
            .join("\n");

        format!("<{}{}>{}</{}>", 
            T::tag_name(),
            class_attr,
            children,
            T::tag_name()
        )
    }
}
