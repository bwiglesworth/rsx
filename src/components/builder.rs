pub struct ComponentBuilder<T: Component> {
    component: T,
    props: T::Props,
    children: Vec<Box<dyn Component>>,
}

impl<T: Component> ComponentBuilder<T> {
    pub fn new(component: T) -> Self {
        Self {
            component,
            props: T::Props::default(),
            children: Vec::new(),
        }
    }

    pub fn with_props(mut self, props: T::Props) -> Self {
        self.props = props;
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> T {
        self.component
    }
}
