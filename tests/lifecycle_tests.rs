use rsx::components::{Component, OnMount, Mountable, LifecycleState};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::pin::Pin;
use std::future::Future;

#[derive(Default)]
struct TestState {
    is_mounted: bool,
}

impl TestState {
    fn set_mounted(&mut self, mounted: bool) {
        self.is_mounted = mounted;
    }

    fn is_mounted(&self) -> bool {
        self.is_mounted
    }
}

struct TestComponent {
    mounted: Arc<Mutex<bool>>,
    state: Arc<Mutex<LifecycleState>>,
    component_state: TestState,
}

impl Component for TestComponent {
    type State = TestState;

    fn render(&self) -> String {
        "<div>Test Component</div>".to_string()
    }

    fn get_state(&self) -> &Self::State {
        &self.component_state
    }

    fn set_state(&mut self, state: Self::State) {
        self.component_state = state;
    }
}

impl OnMount for TestComponent {
    fn on_mount(&self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> {
        let mounted = self.mounted.clone();
        let state = self.state.clone();
        
        Box::pin(async move {
            let mut lock = mounted.lock().await;
            *lock = true;
            
            let mut state_lock = state.lock().await;
            *state_lock = LifecycleState::Mounted;
        })
    }
}

impl Mountable for TestComponent {}
#[tokio::test]
async fn test_component_lifecycle() {
    let mounted = Arc::new(Mutex::new(false));
    let state = Arc::new(Mutex::new(LifecycleState::BeforeMount));
    let mut component_state = TestState::default();
    component_state.set_mounted(true);
    
    let component = TestComponent { 
        mounted: mounted.clone(),
        state: state.clone(),
        component_state,
    };
    
    component.mount();
    
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    
    assert_eq!(*mounted.lock().await, true);
    assert!(matches!(*state.lock().await, LifecycleState::Mounted));
    assert!(component.component_state.is_mounted());
}