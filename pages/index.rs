use rsx::components::Component;

pub struct IndexPage;

impl Component for IndexPage {
    fn render(&self) -> String {
        "<h1>Welcome to RSX!</h1>".to_string()
    }
}
