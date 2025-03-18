pub struct StyleModifier {
    modifiers: Vec<Box<dyn Modifier>>,
    state: ModifierState,
}

impl StyleModifier {
    pub fn new() -> Self {
        Self {
            modifiers: vec![
                Box::new(HoverModifier::new()),
                Box::new(ActiveModifier::new()),
                Box::new(FocusModifier::new()),
                Box::new(MediaModifier::new()),
            ],
            state: ModifierState::default(),
        }
    }

    pub fn modify(&mut self, style: StyleRule) -> ModifiedStyle {
        let mut result = style;
        for modifier in &self.modifiers {
            result = modifier.apply(result, &self.state);
        }
        ModifiedStyle::new(result)
    }
}
