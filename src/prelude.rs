pub use crate::components::{Component, ComponentBuilder};
pub use crate::styles::Style;
pub use crate::props::Props;

pub trait IntoComponent {
    fn into_component(self) -> Box<dyn Component>;
}
