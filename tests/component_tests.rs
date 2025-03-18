use rsx::components::{Component, ComponentBuilder, Card};#[tokio::test]
async fn test_card_component() {
    let card = Card::new(
        "Test Card".to_string(),
        "Test Content".to_string(),
        Vec::new(),
    );

    let card = ComponentBuilder::new(card).build();
    let rendered = card.render();
    
    assert!(rendered.contains("Test Card"));
    assert!(rendered.contains("Test Content"));
}