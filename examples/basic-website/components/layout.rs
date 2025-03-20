pub struct MainLayout {
    content: String,
    show_sidebar: bool,
}

impl MainLayout {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            show_sidebar: true,
        }
    }

    pub fn without_sidebar(mut self) -> Self {
        self.show_sidebar = false;
        self
    }
}

use rsx::Component;

impl Component for MainLayout {
    type State = ();
    
    fn get_state(&self) -> &Self::State {
        &()
    }

    fn set_state(&mut self, _state: Self::State) {}

    fn render(&self) -> String {
        let header = r#"
            <header class="main-header">
                <nav class="navbar">
                    <a href="/" class="navbar-brand">
                        <img src="/assets/img/logo.svg" alt="FarraOx" class="navbar-logo">
                        FarraOx
                    </a>
                    <div class="nav-items">
                        <a href="/" class="nav-link">Home</a>
                        <a href="/about" class="nav-link">About</a>
                    </div>
                </nav>
            </header>"#;

        let sidebar = if self.show_sidebar {
            r#"
            <aside class="sidebar">
                <nav class="sidebar-nav">
                    <h3>Quick Links</h3>
                    <a href="/docs">Documentation</a>
                    <a href="/examples">Examples</a>
                    <a href="/contact">Contact</a>
                </nav>
            </aside>"#
        } else {
            ""
        };

        let footer = r#"
            <footer class="main-footer">
                <p>© 2024 FarraOx. Built with RSX.</p>
            </footer>"#;

        format!(
            r#"
            <div class="layout">
                {header}
                <main class="main-content">
                    {sidebar}
                    <div class="content">
                        {content}
                    </div>
                </main>
                {footer}
            </div>
            "#,
            header = header,
            sidebar = sidebar,
            content = self.content,
            footer = footer
        )
    }
}