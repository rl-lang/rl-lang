use crate::{
    checker::structs::{CheckType, ScopeItem, TypeChecker},
    utils::span::Span,
};

impl TypeChecker {
    // checks if is it declared constant or variable
    // if consant and was declared push error if not then add the constant to the last scope
    // otherwise add the the last scope the new variable
    pub fn declare(&mut self, name: String, item_type: CheckType, is_const: bool, span: Span) {
        if let Some(scope) = self.scopes.last_mut() {
            if is_const && scope.contains_key(&name) {
                self.errors
                    .push(self.err(format!("'{}' is already declared", name), span));
                return;
            }
            scope.insert(name, ScopeItem::new(item_type, is_const));
        }
    }
}
