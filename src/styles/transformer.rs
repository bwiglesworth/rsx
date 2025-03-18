pub struct StyleTransformer {
    transforms: Vec<Box<dyn StyleTransform>>,
    context: TransformContext,
}

impl StyleTransformer {
    pub fn new() -> Self {
        Self {
            transforms: vec![
                Box::new(VariableExpander::new()),
                Box::new(UnitConverter::new()),
                Box::new(ColorTransformer::new()),
                Box::new(MediaQueryTransformer::new())
            ],
            context: TransformContext::default(),
        }
    }

    pub fn transform(&mut self, style: StyleRule) -> TransformedStyle {
        let mut result = style;
        
        for transform in &self.transforms {
            result = transform.apply(result, &self.context);
        }
        
        TransformedStyle {
            rule: result,
            metadata: self.context.metadata.clone(),
        }
    }
}
