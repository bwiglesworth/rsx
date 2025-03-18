use rsx::components::{Component, ComponentBuilder, Counter};

#[tokio::test]
async fn test_counter_component() {
    let counter = Counter::new();
    let counter = ComponentBuilder::new(counter).build();
    let rendered = counter.render();
    
    assert!(rendered.contains("Count:"));
}