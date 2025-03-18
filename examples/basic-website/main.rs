use rsx::components::Component;
use rsx::styles::Style;
use rsx::{Router, Server};
use axum::response::Html;

#[derive(Clone, Default)]
struct HomePageState;

#[derive(Clone)]
struct HomePage {
    state: HomePageState
}

#[derive(Clone, Default)]
struct AboutPageState;

#[derive(Clone)]
struct AboutPage {
    state: AboutPageState
}

impl Component for AboutPage {
    type State = AboutPageState;

    fn render(&self) -> String {
        "<div>About RSX - Building modern web apps in Rust</div>".to_string()
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }
}
impl Component for HomePage {
    type State = HomePageState;

    fn render(&self) -> String {
        "<div>Welcome to RSX!</div>".to_string()
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }
}

#[derive(Clone, Default)]
struct NavBarState;

#[derive(Clone)]
struct NavBar {
    state: NavBarState
}

impl Component for NavBar {
    type State = NavBarState;

    fn render(&self) -> String {
        let mut style = Style::new();
        style.add_rule("nav")
            .property("background", "#333")
            .property("padding", "1rem");

        format!(
            "<style>{}</style>
            <nav>
                <a href='/'>Home</a>
                <a href='/about'>About</a>
            </nav>",
            style
        )
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }
}

#[tokio::main]
async fn main() {
    let navbar = NavBar { state: NavBarState::default() };
    let home = HomePage { state: HomePageState::default() };
    let about = AboutPage { state: AboutPageState::default() };
    
    let mut router = Router::new();
    let navbar_home = navbar.clone();
    let navbar_about = navbar.clone();
    
    router.route("/", move || {
        Html(format!("{}{}", navbar_home.render(), home.render()))
    });
    
    router.route("/about", move || {
        Html(format!("{}{}", navbar_about.render(), about.render()))
    });
    
    let server = Server::new(router);
    server.start().await;}