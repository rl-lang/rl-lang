use std::collections::HashSet;

use ratatui::style::Color;

/// A single documentation entry (leaf node).
pub struct DocItem {
    pub label: String,
    pub tag: &'static str,
    pub tag_color: Color,
    /// Pre-rendered Markdown for this single entry.
    pub content: String,
}

/// A node in the collapsible sidebar tree.
pub enum TreeNode {
    /// A group node (e.g. "Std Reference", "Syntax", "Beginner").
    /// `key` is a unique identifier for tracking expanded state.
    /// `description` is shown in the content pane when the group is selected.
    Group {
        label: String,
        tag: &'static str,
        tag_color: Color,
        key: String,
        description: String,
        children: Vec<TreeNode>,
    },
    /// A leaf node (an actual doc entry).
    Leaf(DocItem),
}

impl TreeNode {
    /// Flatten visible nodes into a Vec for rendering and selection.
    /// Collapsed groups hide their children.
    pub fn flatten_visible(&self, expanded: &HashSet<String>) -> Vec<&TreeNode> {
        let mut out = Vec::new();
        self.flatten_visible_into(expanded, &mut out);
        out
    }

    fn flatten_visible_into<'a>(&'a self, expanded: &HashSet<String>, out: &mut Vec<&'a TreeNode>) {
        match self {
            TreeNode::Leaf(_) => {
                out.push(self);
            }
            TreeNode::Group { key, children, .. } => {
                out.push(self);
                if expanded.contains(key) {
                    for child in children {
                        child.flatten_visible_into(expanded, out);
                    }
                }
            }
        }
    }

    /// Flatten ALL nodes (regardless of expanded state) for search.
    pub fn flatten_all(&self) -> Vec<&TreeNode> {
        let mut out = Vec::new();
        self.flatten_all_into(&mut out);
        out
    }

    fn flatten_all_into<'a>(&'a self, out: &mut Vec<&'a TreeNode>) {
        match self {
            TreeNode::Leaf(_) => {
                out.push(self);
            }
            TreeNode::Group { children, .. } => {
                out.push(self);
                for child in children {
                    child.flatten_all_into(out);
                }
            }
        }
    }

}

/// Which widget currently receives key input.
pub enum Focus {
    List,
    Search,
}
