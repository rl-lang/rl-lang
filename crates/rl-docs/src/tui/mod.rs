mod formatting;
mod main_loop;
mod types;
mod utils;

use crate::entry::{ConceptEntry, StdEntry};

pub fn run_docs_tui(
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
    initial_query: Option<&str>,
) -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = main_loop::run(
        &mut terminal,
        std_entries,
        concept_entries,
        tutorial_entries,
        initial_query,
    );
    ratatui::restore();
    result
}
