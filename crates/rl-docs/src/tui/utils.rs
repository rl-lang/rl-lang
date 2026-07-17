use crate::entry::{ConceptEntry, StdEntry};
use crate::tui::types::DocItem;
use crate::{concept_to_markdown, std_to_markdown, tutorial_to_markdown};
use ratatui::style::Color;

/// Flattens stdlib/concept/tutorial entries into a single browsable list,
/// pre-rendering each entry's Markdown once up front.
pub fn build_items(
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
) -> Vec<DocItem> {
    let mut items = Vec::new();

    for entry in std_entries.iter().copied() {
        items.push(DocItem {
            label: format!("std::{}", entry.name),
            tag: "std",
            tag_color: Color::Cyan,
            content: std_to_markdown(std::slice::from_ref(&entry)),
        });
    }
    for entry in concept_entries.iter().copied() {
        items.push(DocItem {
            label: entry.name.to_string(),
            tag: "concept",
            tag_color: Color::LightBlue,
            content: concept_to_markdown(std::slice::from_ref(&entry)),
        });
    }
    for entry in tutorial_entries.iter().copied() {
        items.push(DocItem {
            label: entry.name.to_string(),
            tag: "tutorial",
            tag_color: Color::Yellow,
            content: tutorial_to_markdown(std::slice::from_ref(&entry)),
        });
    }

    items
}

/// Returns the indices of `items` whose label or tag contains `query`
/// (case-insensitive). Empty query matches everything.
pub fn filter_items(items: &[DocItem], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..items.len()).collect();
    }
    let needle = query.to_lowercase();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            item.label.to_lowercase().contains(&needle) || item.tag.contains(&needle)
        })
        .map(|(i, _)| i)
        .collect()
}
