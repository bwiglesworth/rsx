use rsx::components::{Component, ComponentBuilder, Card, CardState};
#[test]
fn test_card_component() {
    let card = Card {
        state: CardState {
            title: "Test Card".to_string(),
            content: "This is a test card content".to_string(),
        },
        children: Vec::new(),
    };

    let card = ComponentBuilder::new(card)
        .build();

    let rendered = card.render();
    assert!(rendered.contains("Test Card"));
    assert!(rendered.contains("test card content"));
    assert!(rendered.contains(r#"<div class="card">"#));
}