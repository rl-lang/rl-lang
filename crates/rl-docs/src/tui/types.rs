use ratatui::style::Color;

pub struct DocItem {
    pub label: String,
    pub tag: &'static str,
    pub tag_color: Color,
    pub content: String,
}

pub enum Focus {
    List,
    Search,
}
