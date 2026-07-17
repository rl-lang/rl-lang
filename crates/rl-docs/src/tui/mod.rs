//! Interactive TUI docs viewer for `rl docs --tui`.
//!
//! Renders the same stdlib / concept / tutorial entries used by
//! [`crate::std_to_markdown`] and friends, but as a browsable, filterable
//! two-pane app instead of a flat Markdown dump.
//!
//! # Layout
//!
//! ```text
//! |----------------------|--------------------------------------|
//! |  search               |                                      |
//! |----------------------|   content (scrollable, styled from    |
//! |  [std] io             |   the selected entry's Markdown)      |
//! |  [std] math            |                                      |
//! |  [concept] arrays      |                                      |
//! |  [tutorial] step 1     |                                      |
//! |----------------------|--------------------------------------|
//! |  footer / key hints                                          |
//! |----------------------------------------------------------------|
//! ```
//!
//! # Key bindings
//!
//! | Key                                    | Action                       |
//! |-----------------------------------------|------------------------------|
//! | `/`                                     | focus the search box         |
//! | `Enter` / `Esc` (while searching)       | back to the list             |
//! | `↑`/`↓`, `k`/`j`                        | move selection               |
//! | `g` / `G`                               | jump to first / last entry   |
//! | `PageUp`/`PageDown`, `Ctrl+u`/`Ctrl+d`   | scroll the content pane      |
//! | `Esc` (list focused, search non-empty)  | clear the filter             |
//! | `q`, `Ctrl+C`                            | quit                          |

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
