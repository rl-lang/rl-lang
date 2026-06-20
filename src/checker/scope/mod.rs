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
        let found = self.scopes.iter().rev().find_map(|scope| {
            scope
                .get(name)
                .map(|item| (item.type_annotation.clone(), item.is_const))
        });

        if let Some((item_type, is_const)) = found {
            let kind = if is_const { "const" } else { "variable" };
            self.push_hover(
                span,
                format!("```rl\n{} {}: {}\n```", kind, name, item_type.info()),
            );
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
