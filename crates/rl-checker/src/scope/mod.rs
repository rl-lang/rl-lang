//! Scope management and name lookup for the type checker.

mod assign;
mod call;
mod declare;

use crate::structs::{CheckType, TypeChecker};
use rl_utils::{span::Span, suggest::closest_match};

use std::collections::HashMap;

impl TypeChecker {
    /// Pushes a new empty scope onto the scope stack.
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    /// Pops the innermost scope from the stack.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Looks up `name` in all scopes from innermost to outermost.
    ///
    /// On success, pushes a hover entry with the variable's type and kind,
    /// then returns the [`CheckType`]. On failure, emits an undefined variable
    /// error with a "did you mean?" suggestion and returns [`CheckType::Unknown`].
    pub fn lookup(&mut self, name: &str, span: Span) -> CheckType {
        let found = self.scopes.iter().rev().find_map(|scope| {
            scope
                .get(name)
                .map(|item| (item.type_annotation.clone(), item.is_const, item.decl_span))
        });

        if let Some((item_type, is_const, decl_span)) = found {
            let kind = if is_const { "const" } else { "variable" };
            self.push_hover(
                span,
                format!("```rl\n{} {}: {}\n```", kind, name, item_type.info()),
            );
            self.definiations.push((span, decl_span));
            return item_type;
        }

        let all_keys: Vec<String> = self
            .scopes
            .iter()
            .flat_map(|s| s.keys().cloned().collect::<Vec<_>>())
            .collect();
        let suggestion = closest_match(name, all_keys.iter().map(|s| s.as_str()));
        self.error_with_help(format!("undefined variable {}", name), span, suggestion);
        CheckType::Unknown
    }
}
