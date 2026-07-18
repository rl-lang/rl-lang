use crate::references::run_references_spans;
use crate::utils::offset_to_position;

use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, TextEdit, Url, WorkspaceEdit};

pub fn run_rename(
    source: &str,
    position: Position,
    uri: &Url,
    new_name: &str,
) -> Option<WorkspaceEdit> {
    let spans = run_references_spans(source, position, uri, true)?;

    let edits: Vec<TextEdit> = spans
        .into_iter()
        .map(|span| TextEdit {
            range: tower_lsp::lsp_types::Range::new(
                offset_to_position(source, span.start),
                offset_to_position(source, span.end),
            ),
            new_text: new_name.to_string(),
        })
        .collect();

    let mut changes = HashMap::new();
    changes.insert(uri.clone(), edits);

    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}
