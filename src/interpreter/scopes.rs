use std::collections::HashMap;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span, suggest::closest_match},
};

impl Evaluator {
    pub fn push_scope(&mut self) {
        self.environment.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.environment.pop();
    }

    pub fn get_value(&self, name: &str, span: Span) -> Result<Value, Error> {
        for scope in self.environment.iter().rev() {
            if let Some((val, _)) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        let all_keys = self
            .environment
            .iter()
            .flat_map(|s| s.keys().map(|k| k.as_str()));
        let mut err = self.err(format!("undefined variable {}", name), span);
        if let Some(suggestion) = closest_match(name, all_keys) {
            err = err.with_help(format!("did you mean `{}`?", suggestion));
        }
        Err(err)
    }

    pub fn insert_value(&mut self, name: String, value: Value, span: Span) -> Result<(), Error> {
        for scope in self.environment.iter().rev() {
            if let Some((_, true)) = scope.get(&name) {
                return Err(self.err(format!("cannot assign to constant '{}'", name), span));
            }
        }
        if let Some(scope) = self.environment.last_mut() {
            scope.insert(name, (value, false));
        }
        Ok(())
    }

    pub fn insert_const(&mut self, name: String, value: Value, span: Span) -> Result<(), Error> {
        let scope = self.environment.last_mut().unwrap();
        if scope.contains_key(&name) {
            return Err(self.err(format!("'{}' is already declared", name), span));
        }
        scope.insert(name, (value, true));
        Ok(())
    }

    pub fn assign_value(&mut self, name: String, value: Value, span: Span) -> Result<(), Error> {
        for scope in self.environment.iter_mut().rev() {
            if let Some(entry) = scope.get_mut(&name) {
                if entry.1 {
                    return Err(self.err(format!("cannot assign to constant '{}'", name), span));
                }
                entry.0 = value;
                return Ok(());
            }
        }
        Err(self.err(format!("undefined variable '{}'", name), span))
    }
}
