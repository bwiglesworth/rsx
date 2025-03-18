use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StylePriority {
    specificity: (u32, u32, u32),  // (id, class, element)
    source_order: u32,
    importance: bool,
}

impl StylePriority {
    pub fn new(selector: &str, source_order: u32, importance: bool) -> Self {
        let specificity = Self::calculate_specificity(selector);
        Self {
            specificity,
            source_order,
            importance,
        }
    }

    fn calculate_specificity(selector: &str) -> (u32, u32, u32) {
        let id_count = selector.matches('#').count() as u32;
        let class_count = selector.matches('.').count() as u32;
        let element_count = selector.split(' ').count() as u32;
        
        (id_count, class_count, element_count)
    }
}

impl PartialOrd for StylePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
