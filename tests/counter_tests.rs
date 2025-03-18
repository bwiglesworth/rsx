use rsx::components::{Component, ComponentBuilder, Counter, CounterState};

#[test]
fn test_counter_component() {
    let counter = Counter {
        state: CounterState { 
            count: 0,
            updates: 0 
        },
        children: Vec::new(),
    };

    let mut counter = ComponentBuilder::new(counter).build();
    
    assert!(counter.render().contains("Counter: 0"));
    
    counter.set_state(CounterState { 
        count: 5,
        updates: 1 
    });
    assert!(counter.render().contains("Counter: 5"));
    assert!(counter.render().contains("Updates: 1"));
}