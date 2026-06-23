use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{
        evaluator::{EnvironmentItem, Evaluator, PItem},
        values::Value,
    },
    utils::{errors::Error, span::Span, suggest::closest_match},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

impl Evaluator {
    pub fn push_scope(&mut self) {
        self.environment.push(Rc::new(RefCell::new(HashMap::new())));
    }

    pub fn pop_scope(&mut self) {
        self.environment.pop();
    }

    pub fn get_value(&self, name: &str, span: Span) -> Result<Value, Error> {
        for scope in self.environment.iter().rev() {
            if let Some(item) = scope.borrow().get(name) {
                match item {
                    EnvironmentItem::PItem(p) => {
                        return Ok(p.value.clone());
                    }
                }
            }
        }
        let all_keys: Vec<String> = self
            .environment
            .iter()
            .flat_map(|s| s.borrow().keys().cloned().collect::<Vec<_>>())
            .collect();
        let mut err = self.err(format!("undefined variable {}", name), span);
        if let Some(suggestion) = closest_match(name, all_keys.iter().map(|s| s.as_str())) {
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
            if let Some(item) = scope.borrow().get(&name) {
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
            scope.borrow_mut().insert(
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
        let no_scope_err = self.err("no active scope", span);
        let scope = self.environment.last_mut().ok_or(no_scope_err)?;
        if scope.borrow().contains_key(&name) {
            return Err(self.err(format!("'{}' is already declared", name), span));
        }
        scope.borrow_mut().insert(
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
        for scope in self.environment.iter().rev() {
            let mut borrowed_scope = scope.borrow_mut();
            if let Some(entry) = borrowed_scope.get_mut(&name) {
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
                                (TypeAnnotation::Array(a), TypeAnnotation::Array(_))
                                    if **a == TypeAnnotation::Null =>
                                {
                                    true
                                }
                                _ => Evaluator::types_compatible(&value_type, &declared),
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

    // tests
    pub fn get_value_raw(&self, name: &str) -> Option<Value> {
        for scope in self.environment.iter().rev() {
            if let Some(EnvironmentItem::PItem(p)) = scope.borrow().get(name) {
                return Some(p.value.clone());
            }
        }
        None
    }

    // helper function to get the type of declared item
    pub fn get_declared_type(&self, name: &str) -> Option<TypeAnnotation> {
        for scope in self.environment.iter().rev() {
            if let Some(item) = scope.borrow().get(name) {
                match item {
                    EnvironmentItem::PItem(p) => return Some(p.type_annotation.clone()),
                }
            }
        }
        None
    }
}
