//! Variable and constant declaration for the type checker scope.

use crate::structs::{CheckType, ScopeItem, TypeChecker};
use rl_utils::span::Span;

impl TypeChecker {
    /// Declares `name` in the current (innermost) scope.
    ///
    /// For constants, emits an error if the name is already declared in the
    /// same scope. On success, pushes a hover entry showing the kind, name,
    /// and type.
    pub fn declare(&mut self, name: String, item_type: CheckType, is_const: bool, span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            if is_const && scope.contains_key(&name) {
                self.errors
                    .push(self.err(format!("'{}' is already declared", name), span));
                return;
            }

            let kind = if is_const { "const" } else { "variable" };
            let hover_text = format!("```rl\n{} {}: {}\n```", kind, name, item_type.info());

            scope.insert(name, ScopeItem::new(item_type, is_const, span));
            self.push_hover(span, hover_text);
        }
    }
}
