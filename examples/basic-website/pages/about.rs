use crate::components::MainLayout;
use rsx::Component;

pub fn about_page() -> String {
    let content = r#"
        <h1>About FarraOx</h1>
        <p>We're building something amazing.</p>
    "#;
    
    MainLayout::new(content)
        .without_sidebar()
        .render()
}