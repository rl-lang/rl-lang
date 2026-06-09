use ratatui::style::Style;

pub enum OutputLine {
    Input(String),
    Result(String),
    Error(String),
    Info(String),
    Styled(Vec<(String, Style)>),
    Separator,
}
