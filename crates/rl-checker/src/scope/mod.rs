//! Scope management and name lookup for the type checker.

mod assign;
mod call;
mod declare;

use crate::structs::{CheckType, CheckedExpr, TypeChecker};
use rl_ast::statements::Lint;
use rl_utils::{span::Span, suggest::closest_match};

use std::collections::HashMap;

/// Returns "function" or "variable" depending on the item's type.
fn unused_kind(item: &crate::structs::ScopeItem) -> &'static str {
    matches!(item.type_annotation, CheckType::Function { .. })
        .then(|| "function")
        .unwrap_or("variable")
}

impl TypeChecker {
    /// Pushes a new empty scope onto the scope stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    /// Pops the innermost scope from the stack.
    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (name, item) in scope.iter() {
                if !item.used && !item.is_const && !name.starts_with('_')
                    && !item.suppressed_lints.contains(&Lint::Unused)
                {
                    let kind = unused_kind(item);
                    let origin = item.decl_file.clone();
                    self.warn_lint_at(
                        Lint::Unused,
                        format!("unused {} '{}'", kind, name),
                        item.decl_span,
                        origin.as_ref(),
                    );
                }
            }
        }
    }

    /// Reports unused variables in the root (index 0) scope without popping it.
    /// Called at the end of [`TypeChecker::check`] so top-level unused variables
    /// are reported while the scope remains available for post-check inspection.
    pub fn report_unused_in_root_scope(&mut self) {
        if let Some(scope) = self.scopes.first() {
            let unused: Vec<(String, Span, &str, Option<rl_utils::source::SourceFile>)> = scope
                .iter()
                .filter(|(name, item)| {
                    if !item.used && !item.is_const && !name.starts_with('_')
                        && !item.suppressed_lints.contains(&Lint::Unused)
                    {
                        // Skip "main" if no explicit !#[entry] exists -
                        // main is the implicit entry point.
                        if name.as_str() == "main" && !self.has_explicit_entry {
                            return false;
                        }
                        true
                    } else {
                        false
                    }
                })
                .map(|(name, item)| {
                    (
                        name.clone(),
                        item.decl_span,
                        unused_kind(item),
                        item.decl_file.clone(),
                    )
                })
                .collect();
            for (name, span, kind, origin) in unused {
                self.warn_lint_at(
                    Lint::Unused,
                    format!("unused {} '{}'", kind, name),
                    span,
                    origin.as_ref(),
                );
            }
        }
    }

    /// Looks up `name` in all scopes from innermost to outermost.
    ///
    /// On success, pushes a hover entry with the variable's type and kind,
    /// then returns its [`CheckType`] and unit of measure. On failure, emits
    /// an undefined variable error with a "did you mean?" suggestion and
    /// returns [`CheckType::Unknown`] with no unit.
    pub fn lookup(&mut self, name: &str, span: Span) -> CheckedExpr {
        let found = self.scopes.iter_mut().rev().find_map(|scope| {
            scope.get_mut(name).map(|item| {
                item.used = true;
                (
                    item.type_annotation.clone(),
                    item.unit.clone(),
                    item.is_const,
                    item.decl_span,
                    item.deprecated.clone(),
                    item.suppressed_lints.clone(),
                )
            })
        });

        if let Some((item_type, unit, is_const, decl_span, deprecated, suppressed)) = found {
            // Warn on deprecated usage (suppressed by !#[allow(deprecated)])
            if let Some(msg) = &deprecated {
                let text = if msg.is_empty() {
                    format!("'{}' is deprecated", name)
                } else {
                    format!("'{}' is deprecated: {}", name, msg)
                };
                if !suppressed.contains(&Lint::Deprecated) {
                    self.warn_lint(Lint::Deprecated, text, span);
                }
            }

            let kind = if is_const { "const" } else { "variable" };
            let unit_suffix = unit
                .as_ref()
                .map(|u| format!("[{}]", u))
                .unwrap_or_default();
            self.push_hover(
                span,
                format!(
                    "```rl\n{} {}: {}{}\n```",
                    kind,
                    name,
                    item_type.info(),
                    unit_suffix
                ),
            );
            self.definitions.push((span, decl_span));
            return CheckedExpr::new(item_type, unit);
        }

        let all_keys: Vec<String> = self
            .scopes
            .iter()
            .flat_map(|s| s.keys().cloned().collect::<Vec<_>>())
            .collect();
        let suggestion = closest_match(name, all_keys.iter().map(|s| s.as_str()));
        self.error_with_help(format!("undefined variable {}", name), span, suggestion);
        CheckedExpr::new(CheckType::Unknown, None)
    }
}
