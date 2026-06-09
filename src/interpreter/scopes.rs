use std::collections::HashMap;

use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator, PItem},
        values::Value,
    },
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
            if let Some(item) = scope.get(name) {
                match item {
                    EnvironmentItem::PItem(p) => {
                        return Ok(p.value.clone());
                    }
                }
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

    pub fn insert_value(
        &mut self,
        name: String,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        for scope in self.environment.iter().rev() {
            if let Some(item) = scope.get(&name) {
                match item {
                    EnvironmentItem::PItem(p) => {
                        if p.is_const {
                            return Err(
                                self.err(format!("cannot assign to constant '{}'", name), span)
                            );
                        }
                    }
                }
            }
        }
        if let Some(scope) = self.environment.last_mut() {
            scope.insert(
                name,
                EnvironmentItem::PItem(PItem {
                    value,
                    type_annotation,
                    is_const: false,
                }),
            );
        }
        Ok(())
    }

    pub fn insert_const(
        &mut self,
        name: String,
        value: Value,
        type_annotation: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        let scope = self.environment.last_mut().unwrap();
        if scope.contains_key(&name) {
            return Err(self.err(format!("'{}' is already declared", name), span));
        }
        scope.insert(
            name,
            EnvironmentItem::PItem(PItem {
                value,
                type_annotation,
                is_const: true,
            }),
        );
        Ok(())
    }

    pub fn assign_value(
        &mut self,
        name: String,
        value: Value,
        value_type: TypeAnnotation,
        span: Span,
    ) -> Result<(), Error> {
        for scope in self.environment.iter_mut().rev() {
            if let Some(entry) = scope.get_mut(&name) {
                match entry {
                    EnvironmentItem::PItem(p) => {
                        if p.is_const {
                            return Err(
                                self.err(format!("cannot assign to constant '{}'", name), span)
                            );
                        }
                        let declared = p.type_annotation.clone();

                        // Null is assignable to any type (implicit nullability).
                        // Type-checking against the declared type is skipped when
                        // the incoming value is Null — the null-use error fires
                        // later at the point of use instead.
                        let types_match = matches!(value, Value::Null)
                            || match (&declared, &value_type) {
                                (TypeAnnotation::Array(_), TypeAnnotation::Array(inner))
                                    if **inner == TypeAnnotation::Null =>
                                {
                                    true
                                }
                                _ => declared == value_type,
                            };
                        if !types_match {
                            return Err(self.err(
                                format!(
                                    "cannot assign {:?} to variable '{}' declared as {:?}",
                                    value_type, name, declared
                                ),
                                span,
                            ));
                        }

                        p.value = value;
                        return Ok(());
                    }
                }
            }
        }
        Err(self.err(format!("undefined variable '{}'", name), span))
    }
}
