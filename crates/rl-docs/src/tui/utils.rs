use std::collections::{HashMap, HashSet};

use crate::entry::{ConceptCategory, ConceptEntry, FnEntry, StdEntry};
use crate::tui::types::{DocItem, TreeNode};
use crate::{concept_to_markdown, std_to_markdown, tutorial_to_markdown};
use ratatui::style::Color;

pub fn build_tree(
    std_entries: &[&StdEntry],
    concept_entries: &[&ConceptEntry],
    tutorial_entries: &[&ConceptEntry],
) -> TreeNode {
    let children = vec![
        build_std_tree(std_entries),
        build_concepts_tree(concept_entries),
        build_tutorial_tree(tutorial_entries),
    ];

    TreeNode::Group {
        label: String::new(),
        tag: "",
        tag_color: Color::White,
        key: "root".to_string(),
        description: "rl docs - browse the standard library, language concepts, and tutorials.\n\n\
            Use ↑/↓ or j/k to navigate, Enter/→ to expand, ← to collapse.\n\
            Press / to search."
            .to_string(),
        children,
    }
}

fn build_std_tree(entries: &[&StdEntry]) -> TreeNode {
    let mut top_level: Vec<&&StdEntry> = Vec::new();
    let mut children_map: HashMap<String, Vec<&StdEntry>> = HashMap::new();

    for entry in entries {
        if entry.name.contains("::") {
            children_map
                .entry(entry.name.to_string())
                .or_default()
                .push(entry);
        } else {
            top_level.push(entry);
        }
    }

    let mut std_children: Vec<TreeNode> = Vec::new();

    for entry in top_level {
        let sub_key = entry.name;

        let subs: Vec<&StdEntry> = children_map
            .iter()
            .filter(|(k, _)| k.starts_with(&format!("{}::", sub_key)))
            .flat_map(|(_, v)| v.iter().copied())
            .collect();

        if subs.is_empty() {
            // Module with only functions
            if entry.functions.is_empty() {
                std_children.push(TreeNode::Leaf(DocItem {
                    label: entry.name.to_string(),
                    tag: "std",
                    tag_color: Color::Cyan,
                    content: std_to_markdown(std::slice::from_ref(entry)),
                }));
            } else {
                let func_children: Vec<TreeNode> = entry
                    .functions
                    .iter()
                    .map(|func| {
                        TreeNode::Leaf(DocItem {
                            label: func_name(func).to_string(),
                            tag: "std",
                            tag_color: Color::Cyan,
                            content: render_function(entry, func),
                        })
                    })
                    .collect();

                std_children.push(TreeNode::Group {
                    label: entry.name.to_string(),
                    tag: "std",
                    tag_color: Color::Cyan,
                    key: format!("std::{}", entry.name),
                    description: format!(
                        "**std::{}** - {}\n\n{}",
                        entry.name,
                        entry.description,
                        module_summary(entry)
                    ),
                    children: func_children,
                });
            }
        } else {
            // Module with sub-modules (e.g. math → math::consts)
            let mut group_children: Vec<TreeNode> = Vec::new();

            // Overview leaf for the parent module
            group_children.push(TreeNode::Leaf(DocItem {
                label: format!("{} (overview)", entry.name),
                tag: "std",
                tag_color: Color::Cyan,
                content: std_to_markdown(std::slice::from_ref(entry)),
            }));

            for sub in &subs {
                let sub_name = sub.name.rsplit("::").next().unwrap_or(sub.name);
                if sub.functions.is_empty() {
                    group_children.push(TreeNode::Leaf(DocItem {
                        label: sub_name.to_string(),
                        tag: "std",
                        tag_color: Color::Cyan,
                        content: std_to_markdown(std::slice::from_ref(sub)),
                    }));
                } else {
                    let func_children: Vec<TreeNode> = sub
                        .functions
                        .iter()
                        .map(|func| {
                            TreeNode::Leaf(DocItem {
                                label: func_name(func).to_string(),
                                tag: "std",
                                tag_color: Color::Cyan,
                                content: render_function(sub, func),
                            })
                        })
                        .collect();

                    group_children.push(TreeNode::Group {
                        label: sub_name.to_string(),
                        tag: "std",
                        tag_color: Color::Cyan,
                        key: format!("std::{}", sub.name),
                        description: format!(
                            "**std::{}** - {}\n\n{}",
                            sub.name,
                            sub.description,
                            module_summary(sub)
                        ),
                        children: func_children,
                    });
                }
            }

            std_children.push(TreeNode::Group {
                label: entry.name.to_string(),
                tag: "std",
                tag_color: Color::Cyan,
                key: format!("std::{}", entry.name),
                description: format!(
                    "**std::{}** - {}\n\n{}",
                    entry.name,
                    entry.description,
                    module_summary(entry)
                ),
                children: group_children,
            });
        }
    }

    TreeNode::Group {
        label: "Std Reference".to_string(),
        tag: "std",
        tag_color: Color::Cyan,
        key: "std".to_string(),
        description: "Standard library modules and functions.\n\n\
            Expand a module to see its functions, or select a function to view its documentation."
            .to_string(),
        children: std_children,
    }
}

fn func_name(func: &FnEntry) -> &str {
    func.signature.split('(').next().unwrap_or(func.signature)
}

fn module_summary(module: &StdEntry) -> String {
    if module.functions.is_empty() {
        return String::new();
    }
    let names: Vec<&str> = module.functions.iter().map(|f| func_name(f)).collect();
    format!("Functions: {}", names.join(", "))
}

fn render_function(module: &StdEntry, func: &FnEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!("# std::{}::{}\n\n", module.name, func_name(func)));
    if let Some(since) = func.since {
        out.push_str(&format!("*since {}*", since));
        if let Some(updated) = func.updated {
            out.push_str(&format!(" *updated {}*", updated));
        }
        out.push_str("\n\n");
    }
    if let Some(deprecated) = func.deprecated {
        out.push_str(&format!("**Deprecated:** {}\n\n", deprecated));
    }
    out.push_str(&format!("{}\n\n", func.description));
    out.push_str(&format!("**Returns:** {}\n\n", func.returns));
    if let Some(errors) = func.errors {
        out.push_str(&format!("**Errors:** {}\n\n", errors));
    }
    out.push_str(&format!("```rl\n{}\n```\n\n", func.example));
    if let Some(expected) = func.expected_output {
        out.push_str(&format!("output:\n```text\n{}\n```\n\n", expected));
    }
    if !func.see_also.is_empty() {
        out.push_str("**See also:** ");
        out.push_str(&func.see_also.join(", "));
        out.push_str("\n\n");
    }
    out
}

fn build_concepts_tree(entries: &[&ConceptEntry]) -> TreeNode {
    let mut by_category: HashMap<ConceptCategory, Vec<&ConceptEntry>> = HashMap::new();
    for entry in entries {
        by_category
            .entry(entry.category.clone())
            .or_default()
            .push(entry);
    }

    let category_order = [
        ConceptCategory::Syntax,
        ConceptCategory::Types,
        ConceptCategory::ControlFlow,
        ConceptCategory::Functions,
        ConceptCategory::Modules,
        ConceptCategory::ErrorHandling,
        ConceptCategory::Tooling,
    ];

    let cat_descriptions: HashMap<&str, &str> = HashMap::from([
        ("Syntax", "Comments, literals, variables, and basic language structure."),
        ("Types", "Type system: casting, annotations, Result[T], and more."),
        (
            "Control Flow",
            "Branching and looping: if/elif/else, for, while.",
        ),
        (
            "Functions",
            "Function definitions, lambdas, and closures.",
        ),
        ("Modules", "Imports and the module system."),
        (
            "Error Handling",
            "Errors, Result[T], and failure handling patterns.",
        ),
        (
            "Tooling",
            "CLI, LSP, REPL, and other developer tools.",
        ),
    ]);

    let mut concept_children: Vec<TreeNode> = Vec::new();
    for cat in &category_order {
        if let Some(cat_entries) = by_category.get(cat) {
            let cat_label = cat.to_string();
            let desc = cat_descriptions
                .get(cat_label.as_str())
                .unwrap_or(&"");
            let children: Vec<TreeNode> = cat_entries
                .iter()
                .map(|entry| {
                    TreeNode::Leaf(DocItem {
                        label: entry.name.to_string(),
                        tag: "concept",
                        tag_color: Color::LightBlue,
                        content: concept_to_markdown(std::slice::from_ref(entry)),
                    })
                })
                .collect();

            concept_children.push(TreeNode::Group {
                label: cat_label.clone(),
                tag: "concept",
                tag_color: Color::LightBlue,
                key: format!("concept::{}", cat_label),
                description: format!("**{}**\n\n{}", cat_label, desc),
                children,
            });
        }
    }

    TreeNode::Group {
        label: "Concepts".to_string(),
        tag: "concept",
        tag_color: Color::LightBlue,
        key: "concepts".to_string(),
        description: "Language concepts and syntax reference.\n\n\
            Each category groups related concepts together."
            .to_string(),
        children: concept_children,
    }
}

fn build_tutorial_tree(entries: &[&ConceptEntry]) -> TreeNode {
    // Split by where numbering resets (end of t1 → start of t2)
    let mut split_idx = entries.len();
    for i in 1..entries.len() {
        if entries[i].name.starts_with("1. ") && !entries[i - 1].name.starts_with("1. ") {
            split_idx = i;
            break;
        }
    }

    let beginner = &entries[..split_idx];
    let advanced = &entries[split_idx..];

    let beginner_children: Vec<TreeNode> = beginner
        .iter()
        .map(|entry| {
            TreeNode::Leaf(DocItem {
                label: entry.name.to_string(),
                tag: "tutorial",
                tag_color: Color::Yellow,
                content: tutorial_to_markdown(std::slice::from_ref(entry)),
            })
        })
        .collect();

    let advanced_children: Vec<TreeNode> = advanced
        .iter()
        .map(|entry| {
            TreeNode::Leaf(DocItem {
                label: entry.name.to_string(),
                tag: "tutorial",
                tag_color: Color::Yellow,
                content: tutorial_to_markdown(std::slice::from_ref(entry)),
            })
        })
        .collect();

    let mut tutorial_children = Vec::new();
    if !beginner_children.is_empty() {
        tutorial_children.push(TreeNode::Group {
            label: "Beginner".to_string(),
            tag: "tutorial",
            tag_color: Color::Yellow,
            key: "tutorial::beginner".to_string(),
            description: "Beginner tutorial - learn rl from scratch, step by step.\n\n\
                Covers variables, types, I/O, control flow, functions, arrays, and more."
                .to_string(),
            children: beginner_children,
        });
    }
    if !advanced_children.is_empty() {
        tutorial_children.push(TreeNode::Group {
            label: "Advanced".to_string(),
            tag: "tutorial",
            tag_color: Color::Yellow,
            key: "tutorial::advanced".to_string(),
            description: "Advanced tutorial - build a complete CSV query tool.\n\n\
                Covers modules, file I/O, string parsing, and building a real program."
                .to_string(),
            children: advanced_children,
        });
    }

    TreeNode::Group {
        label: "Tutorial".to_string(),
        tag: "tutorial",
        tag_color: Color::Yellow,
        key: "tutorial".to_string(),
        description: "Step-by-step tutorials to learn rl.\n\n\
            Beginner: start here if you're new to rl.\n\
            Advanced: build a real-world project."
            .to_string(),
        children: tutorial_children,
    }
}

pub fn filter_tree(tree: &TreeNode, expanded: &HashSet<String>, query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..tree.flatten_visible(expanded).len()).collect();
    }

    let needle = query.to_lowercase();
    let all_nodes = tree.flatten_all();

    let matching_leaves: HashSet<usize> = all_nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| match node {
            TreeNode::Leaf(item) => {
                item.label.to_lowercase().contains(&needle)
                    || item.tag.to_lowercase().contains(&needle)
            }
            TreeNode::Group { label, tag, .. } => {
                label.to_lowercase().contains(&needle)
                    || tag.to_lowercase().contains(&needle)
            }
        })
        .map(|(i, _)| i)
        .collect();

    let mut auto_expand_keys = HashSet::new();
    for &idx in &matching_leaves {
        let mut path = Vec::new();
        if all_nodes[idx].ancestor_keys_from(tree, &mut path) {
            for key in path {
                auto_expand_keys.insert(key);
            }
        }
    }

    let mut effective_expanded = expanded.clone();
    for key in auto_expand_keys {
        effective_expanded.insert(key);
    }

    let visible = tree.flatten_visible(&effective_expanded);
    visible
        .iter()
        .enumerate()
        .filter(|(_, node)| match node {
            TreeNode::Leaf(item) => {
                item.label.to_lowercase().contains(&needle)
                    || item.tag.to_lowercase().contains(&needle)
            }
            TreeNode::Group { label, tag, .. } => {
                label.to_lowercase().contains(&needle)
                    || tag.to_lowercase().contains(&needle)
            }
        })
        .map(|(i, _)| i)
        .collect()
}

trait AncestorKeys {
    fn ancestor_keys_from(&self, root: &TreeNode, path: &mut Vec<String>) -> bool;
}

impl AncestorKeys for TreeNode {
    fn ancestor_keys_from(&self, root: &TreeNode, path: &mut Vec<String>) -> bool {
        match self {
            TreeNode::Leaf(target) => find_ancestors(root, target, path),
            _ => false,
        }
    }
}

fn find_ancestors(node: &TreeNode, target: &DocItem, path: &mut Vec<String>) -> bool {
    match node {
        TreeNode::Leaf(item) => std::ptr::eq(item as *const DocItem, target as *const DocItem),
        TreeNode::Group { key, children, .. } => {
            path.push(key.clone());
            for child in children {
                if find_ancestors(child, target, path) {
                    return true;
                }
            }
            path.pop();
            false
        }
    }
}
