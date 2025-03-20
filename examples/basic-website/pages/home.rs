use crate::components::MainLayout;
use rsx::Component;

pub fn home_page() -> String {
    let content = r#"
        <h1>Welcome to FarraOx!</h1>
        <p>This is our homepage.</p>
    "#;
    
    MainLayout::new(content).render()
}