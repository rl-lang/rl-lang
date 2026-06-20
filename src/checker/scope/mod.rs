mod assign;
mod call;
mod declare;

use crate::{
    checker::structs::{CheckType, TypeChecker},
    utils::{span::Span, suggest::closest_match},
};
use std::collections::HashMap;

impl TypeChecker {
    // adds or removes scope from/to TypeChecker scopes
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    // checks current scope for the said identifier
    pub fn lookup(&mut self, name: &str, span: Span) -> CheckType {
        for scope in self.scopes.iter().rev() {
            if let Some(item) = scope.get(name) {
                return item.type_annotation.clone();
            }
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
