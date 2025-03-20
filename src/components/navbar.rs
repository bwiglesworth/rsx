use crate::{Component, Style};

pub struct NavItem {
    text: String,
    href: String,
    active: bool,
}

pub struct Navbar {
    brand: String,
    items: Vec<NavItem>,
    theme: NavTheme,
}

pub enum NavTheme {
    Light,
    Dark,
    Custom(String),
}

impl Navbar {
    pub fn new(brand: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            items: Vec::new(),
            theme: NavTheme::Light,
        }
    }

    pub fn add_item(&mut self, text: impl Into<String>, href: impl Into<String>) -> &mut Self {
        self.items.push(NavItem {
            text: text.into(),
            href: href.into(),
            active: false,
        });
        self
    }

    pub fn set_theme(&mut self, theme: NavTheme) -> &mut Self {
        self.theme = theme;
        self
    }
}

impl Component for Navbar {
    fn render(&self) -> String {
        let mut style = Style::new();
        style.add_rule(".navbar")
            .property("display", "flex")
            .property("align-items", "center")
            .property("padding", "1rem")
            .property("background-color", match self.theme {
                NavTheme::Light => "#ffffff",
                NavTheme::Dark => "#333333",
                NavTheme::Custom(ref color) => color,
            });

        style.add_rule(".navbar-brand")
            .property("font-size", "1.5rem")
            .property("font-weight", "bold")
            .property("text-decoration", "none")
            .property("color", match self.theme {
                NavTheme::Light => "#333333",
                NavTheme::Dark => "#ffffff",
                NavTheme::Custom(_) => "#333333",
            });

        style.add_rule(".navbar-items")
            .property("display", "flex")
            .property("gap", "1rem")
            .property("margin-left", "2rem");

        style.add_rule(".nav-item")
            .property("text-decoration", "none")
            .property("color", match self.theme {
                NavTheme::Light => "#666666",
                NavTheme::Dark => "#dddddd",
                NavTheme::Custom(_) => "#666666",
            });

        let items_html = self.items.iter()
            .map(|item| format!(
                r#"<a href="{}" class="nav-item">{}</a>"#,
                item.href, item.text
            ))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"
            <nav class="navbar">
                {style}
                <a href="/" class="navbar-brand">{}</a>
                <div class="navbar-items">
                    {items_html}
                </div>
            </nav>
            "#,
            self.brand
        )
    }
}
