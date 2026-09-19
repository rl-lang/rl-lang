use std::collections::HashSet;

use crate::{
    entry::{ConceptEntry, StdEntry},
    tui::{
        formatting::markdown_to_lines,
        types::{DocItem, Focus, TreeNode},
        utils::{build_tree, filter_tree},
    },
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

pub fn run(
    terminal: &mut DefaultTerminal,
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
    initial_query: Option<&str>,
) -> std::io::Result<()> {
    let tree = build_tree(std_entries, concept_entries, tutorial_entries);
    let mut expanded: HashSet<String> = HashSet::new();
    // Start with the three top-level categories expanded
    expanded.insert("std".to_string());
    expanded.insert("concepts".to_string());
    expanded.insert("tutorial".to_string());

    let mut query = initial_query.unwrap_or("").to_string();
    let mut focus = Focus::List;
    let mut selected: usize = 0;
    let mut content_scroll: u16 = 0;
    let mut list_state = ListState::default();

    // We need to track which DocItem is selected for content display.
    // The filtered list gives indices into the flattened visible list.
    // We need to find the corresponding DocItem.
    #[allow(unused_assignments)]
    let mut selected_item: Option<DocItem> = None;

    loop {
        let filtered = filter_tree(&tree, &expanded, &query);
        if filtered.is_empty() {
            selected = 0;
        } else if selected >= filtered.len() {
            selected = filtered.len() - 1;
        }

        // Find the selected DocItem
        let visible = tree.flatten_visible(&expanded);
        selected_item = filtered.get(selected).and_then(|&idx| {
            visible.get(idx).map(|node| match node {
                TreeNode::Leaf(item) => DocItem {
                    label: item.label.clone(),
                    tag: item.tag,
                    tag_color: item.tag_color,
                    content: item.content.clone(),
                },
                TreeNode::Group {
                    label,
                    tag,
                    tag_color,
                    description,
                    ..
                } => DocItem {
                    label: label.clone(),
                    tag,
                    tag_color: *tag_color,
                    content: description.clone(),
                },
            })
        });

        list_state.select(if filtered.is_empty() {
            None
        } else {
            Some(selected)
        });

        terminal.draw(|frame| {
            let area = frame.area();
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(1)])
                .split(area);

            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(32), Constraint::Percentage(68)])
                .split(outer[0]);

            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(3)])
                .split(cols[0]);

            // search box
            let search_border = match focus {
                Focus::Search => Style::default().fg(Color::Cyan),
                Focus::List => Style::default().fg(Color::DarkGray),
            };
            let search_line = if query.is_empty() && matches!(focus, Focus::List) {
                Line::from(Span::styled(
                    "/ to search...",
                    Style::default().fg(Color::DarkGray),
                ))
            } else {
                let mut spans = vec![Span::styled(
                    query.clone(),
                    Style::default().fg(Color::White),
                )];
                if matches!(focus, Focus::Search) {
                    spans.push(Span::styled("│", Style::default().fg(Color::Cyan)));
                }
                Line::from(spans)
            };
            let search_widget = Paragraph::new(search_line).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(search_border)
                    .title(Span::styled(
                        " search ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )),
            );
            frame.render_widget(search_widget, left[0]);

            // sidebar tree
            let list_items: Vec<ListItem> = filtered
                .iter()
                .filter_map(|&idx| visible.get(idx))
                .map(|node| {
                    let depth = compute_depth(node, &tree);
                    let indent = "  ".repeat(depth);
                    match node {
                        TreeNode::Group {
                            label, tag_color, ..
                        } => {
                            let is_expanded = expanded.contains(&get_key(node));
                            let marker = if is_expanded { "▼ " } else { "▶ " };
                            ListItem::new(Line::from(vec![
                                Span::styled(
                                    format!("{}{}", indent, marker),
                                    Style::default().fg(Color::DarkGray),
                                ),
                                Span::styled(
                                    label.clone(),
                                    Style::default()
                                        .fg(*tag_color)
                                        .add_modifier(Modifier::BOLD),
                                ),
                            ]))
                        }
                        TreeNode::Leaf(item) => {
                            ListItem::new(Line::from(vec![
                                Span::styled(
                                    format!("{}  ", indent),
                                    Style::default().fg(Color::DarkGray),
                                ),
                                Span::styled(
                                    item.label.clone(),
                                    Style::default().fg(item.tag_color),
                                ),
                            ]))
                        }
                    }
                })
                .collect();

            let list_widget = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(Span::styled(
                            format!(" entries ({}) ", filtered.len()),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_stateful_widget(list_widget, left[1], &mut list_state);

            // content pane
            let content_lines = match &selected_item {
                Some(item) => markdown_to_lines(&item.content),
                None => vec![Line::from(Span::styled(
                    "select an entry",
                    Style::default().fg(Color::DarkGray),
                ))],
            };
            let content_title = selected_item
                .as_ref()
                .map(|item| format!(" {} ", item.label))
                .unwrap_or_else(|| " docs ".to_string());
            let content_widget = Paragraph::new(content_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::DarkGray))
                        .title(Span::styled(
                            content_title,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )),
                )
                .wrap(Wrap { trim: false });

            let visible_height = cols[1].height.saturating_sub(2);
            let total = content_widget.line_count(cols[1].width) as u16;
            let max_scroll = total.saturating_sub(visible_height);
            if content_scroll > max_scroll {
                content_scroll = max_scroll;
            }
            let content_widget = content_widget.scroll((content_scroll, 0));
            frame.render_widget(content_widget, cols[1]);

            // footer
            let footer_text = match focus {
                Focus::Search => "type to filter  •  Enter/Esc back to list  •  Ctrl+C quit",
                Focus::List => {
                    "/ search  •  ↑↓/jk select  •  Enter/→/l expand  •  ←/h collapse  •  PgUp/PgDn scroll  •  q quit"
                }
            };
            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    footer_text,
                    Style::default().fg(Color::DarkGray),
                ))),
                outer[1],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            match focus {
                Focus::Search => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (_, KeyCode::Enter) => {
                        focus = Focus::List;
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Esc) => {
                        query.clear();
                        focus = Focus::List;
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Backspace) => {
                        query.pop();
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Char(c)) => {
                        query.push(c);
                        selected = 0;
                        content_scroll = 0;
                    }
                    _ => {}
                },
                Focus::List => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (_, KeyCode::Char('q')) => break,
                    (_, KeyCode::Char('/')) => focus = Focus::Search,
                    (_, KeyCode::Esc) if !query.is_empty() => {
                        query.clear();
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                        selected = selected.saturating_sub(1);
                        content_scroll = 0;
                    }
                    (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                        selected = selected.saturating_add(1);
                        content_scroll = 0;
                    }
                    (_, KeyCode::Enter) | (_, KeyCode::Right) | (_, KeyCode::Char('l')) => {
                        // Toggle expand/collapse on the selected group
                        if let Some(&idx) = filtered.get(selected)
                            && let Some(TreeNode::Group { key, .. }) = visible.get(idx)
                        {
                            let key = key.clone();
                            if expanded.contains(&key) {
                                expanded.remove(&key);
                            } else {
                                expanded.insert(key);
                            }
                            content_scroll = 0;
                        }
                    }
                    (_, KeyCode::Left) | (_, KeyCode::Char('h')) => {
                        // Collapse the selected group, or collapse its parent
                        if let Some(&idx) = filtered.get(selected)
                            && let Some(node) = visible.get(idx)
                        {
                            match node {
                                TreeNode::Group { key, .. } => {
                                    let key = key.clone();
                                    expanded.remove(&key);
                                }
                                TreeNode::Leaf(_) => {
                                    // Find parent group and collapse it
                                    if let Some(parent_key) = find_parent_key(&tree, idx) {
                                        expanded.remove(&parent_key);
                                    }
                                }
                            }
                            content_scroll = 0;
                        }
                    }
                    (_, KeyCode::Char('g')) => {
                        selected = 0;
                        content_scroll = 0;
                    }
                    (_, KeyCode::Char('G')) => {
                        selected = usize::MAX;
                        content_scroll = 0;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('d')) | (_, KeyCode::PageDown) => {
                        content_scroll = content_scroll.saturating_add(10);
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('u')) | (_, KeyCode::PageUp) => {
                        content_scroll = content_scroll.saturating_sub(10);
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(())
}

/// Get the key of a group node.
fn get_key(node: &TreeNode) -> String {
    match node {
        TreeNode::Group { key, .. } => key.clone(),
        TreeNode::Leaf(_) => String::new(),
    }
}

/// Compute the depth of a node in the tree (for indentation).
fn compute_depth(target: &TreeNode, root: &TreeNode) -> usize {
    fn find_depth(node: &TreeNode, target: &TreeNode, depth: usize) -> Option<usize> {
        // Compare by pointer identity for groups, or by label for leaves
        match (node, target) {
            (
                TreeNode::Group {
                    key: k1, children, ..
                },
                TreeNode::Group {
                    key: k2, ..
                },
            ) => {
                if k1 == k2 {
                    return Some(depth);
                }
                for child in children {
                    if let Some(d) = find_depth(child, target, depth + 1) {
                        return Some(d);
                    }
                }
                None
            }
            (TreeNode::Group { children, .. }, TreeNode::Leaf(tgt_item)) => {
                for child in children {
                    match child {
                        TreeNode::Leaf(item)
                            if std::ptr::eq(item as *const DocItem, tgt_item as *const DocItem) =>
                        {
                            return Some(depth + 1);
                        }
                        TreeNode::Group { children: _sub_children, .. } => {
                            if let Some(d) = find_depth(child, target, depth + 1) {
                                return Some(d);
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }

    find_depth(root, target, 0).unwrap_or(0)
}

/// Find the parent group key for a visible index.
fn find_parent_key(tree: &TreeNode, target_idx: usize) -> Option<String> {
    let visible = tree.flatten_visible(&HashSet::new());
    if target_idx >= visible.len() {
        return None;
    }

    // Walk the tree to find the parent of the node at target_idx
    fn find_parent(
        node: &TreeNode,
        target_idx: usize,
        current_idx: &mut usize,
        parent_key: &mut Option<String>,
    ) -> bool {
        match node {
            TreeNode::Leaf(_) => {
                *current_idx += 1;
                *current_idx - 1 == target_idx
            }
            TreeNode::Group {
                key, children, ..
            } => {
                let _group_idx = *current_idx;
                *current_idx += 1;

                let saved_parent = parent_key.clone();
                *parent_key = Some(key.clone());

                for child in children {
                    if find_parent(child, target_idx, current_idx, parent_key) {
                        return true;
                    }
                }

                *parent_key = saved_parent;
                false
            }
        }
    }

    let mut current_idx = 0;
    let mut parent_key = None;
    find_parent(tree, target_idx, &mut current_idx, &mut parent_key);
    parent_key
}
