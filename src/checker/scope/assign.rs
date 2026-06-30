//! Assignment type checking - validates reassignment against the declared type.

use crate::{
    checker::structs::{CheckType, TypeChecker},
    utils::{span::Span, suggest::closest_match},
};

impl TypeChecker {
    /// Checks that assigning `value_type` to `name` is valid.
    ///
    /// Walks scopes from innermost to outermost. Emits an error if:
    /// - The variable is declared `const`
    /// - The value type doesn't match the declared type
    /// - The name is not declared in any scope (with a "did you mean?" suggestion)
    pub fn assign(&mut self, name: &str, value_type: CheckType, span: Span) {
        let mut const_error: Option<String> = None;
        let mut type_error: Option<String> = None;
        let mut found = false;

        for scope in self.scopes.iter_mut().rev() {
            if let Some(item) = scope.get_mut(name) {
                found = true;

                if item.is_const {
                    const_error = Some(format!("cannot assign to constant '{}'", name));
                } else if !value_type.matches(&item.type_annotation) && !value_type.is_null() {
                    type_error = Some(format!(
                        "cannot assign {} to variable '{}' declared as {}",
                        value_type.info(),
                        name,
                        item.type_annotation.info(),
                    ));
                }
                break;
            }
        }

        if let Some(msg) = const_error.or(type_error) {
            self.error(msg, span);
            return;
        }

        if !found {
            let all_keys: Vec<String> = self
                .scopes
                .iter()
                .flat_map(|s| s.keys().cloned().collect::<Vec<_>>())
                .collect();
            let suggestion = closest_match(name, all_keys.iter().map(|s| s.as_str()));

            self.error_with_help(format!("undefined variable '{}'", name), span, suggestion);
        }
    }
}
