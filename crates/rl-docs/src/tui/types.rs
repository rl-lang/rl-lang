use ratatui::style::Color;

/// One flattened, browsable row in the sidebar list.
pub struct DocItem {
    pub label: String,
    pub tag: &'static str,
    pub tag_color: Color,
    /// Pre-rendered Markdown for this single entry, reusing the same
    /// renderers as the non-interactive `rl docs` output.
    pub content: String,
}

/// Which widget currently receives key input.
pub enum Focus {
    List,
    Search,
}
