use super::Component;

#[derive(Clone, Default)]
pub struct ButtonState {
    // State fields for the button
    pub disabled: bool,
    pub clicked: bool,
}

pub struct Button {
    text: String,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    state: ButtonState,
    class_name: String,
}

impl Component for Button {
    type State = ButtonState;

    fn render(&self) -> String {
        let disabled_attr = if self.state.disabled { " disabled" } else { "" };
        let class_attr = if !self.class_name.is_empty() { 
            format!(" class=\"{}\"", self.class_name) 
        } else { 
            String::new() 
        };
        
        format!(
            r#"<button{}{} onclick="handleButtonClick(this)">{}</button>"#,
            class_attr,
            disabled_attr,
            self.text
        )
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }

    fn updated(&mut self) {
        // Handle any post-update logic here
        if self.state.clicked {
            if let Some(on_click) = &self.on_click {
                on_click();
            }
            // Reset clicked state after handling
            self.state.clicked = false;
        }
    }
}

impl Button {
    pub fn new(text: String) -> Self {
        Self {
            text,
            on_click: None,
            state: ButtonState::default(),
            class_name: String::new(),
        }
    }

    pub fn with_class(mut self, class_name: String) -> Self {
        self.class_name = class_name;
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self 
    where 
        F: Fn() + Send + Sync + 'static 
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.disabled = disabled;
        self
    }

    pub fn click(&mut self) {
        if !self.state.disabled {
            let mut new_state = self.state.clone();
            new_state.clicked = true;
            self.set_state(new_state);
            self.updated();
        }
    }
}
