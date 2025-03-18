pub struct StyleOptimizer {
    rules: Vec<CompiledRule>,
    optimizations: Vec<Box<dyn StyleOptimization>>,
}

impl StyleOptimizer {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            optimizations: vec![
                Box::new(DuplicateRemoval::new()),
                Box::new(PropertyMerger::new()),
                Box::new(SelectorMinifier::new()),
            ],
        }
    }

    pub fn optimize(&mut self, css: String) -> String {
        let mut optimized = css;
        for optimization in &self.optimizations {
            optimized = optimization.apply(&optimized);
        }
        optimized
    }
}

trait StyleOptimization {
    fn apply(&self, css: &str) -> String;
}
