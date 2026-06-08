use std::sync::Arc;

use crate::{
    ast::statements::{Statement, StatementKind, TypeAnnotation},
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.kind {
            StatementKind::VariableDeclaration { name, value, .. } => {
                let val = self.evaluate(value)?;
                let inferred_type = match &val {
                    Value::Integer(_) => TypeAnnotation::Int,
                    Value::Float(_) => TypeAnnotation::Float,
                    Value::String(_) => TypeAnnotation::String,
                    Value::Bool(_) => TypeAnnotation::Bool,
                    Value::Char(_) => TypeAnnotation::Char,
                    Value::Values(items) => {
                        let inner = items
                            .first()
                            .map(|v| match v {
                                Value::Integer(_) => TypeAnnotation::Int,
                                Value::Float(_) => TypeAnnotation::Float,
                                Value::String(_) => TypeAnnotation::String,
                                Value::Bool(_) => TypeAnnotation::Bool,
                                Value::Char(_) => TypeAnnotation::Char,
                                _ => TypeAnnotation::Null,
                            })
                            .unwrap_or(TypeAnnotation::Null);
                        TypeAnnotation::Array(Box::new(inner))
                    }
                    Value::Null => TypeAnnotation::Null,
                    Value::Function { .. } => TypeAnnotation::Fn,
                };

                self.insert_value(name.clone(), val, inferred_type, statement.span)?;
            }

            StatementKind::Array { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                let inner = items
                    .first()
                    .map(|v| match v {
                        Value::Integer(_) => TypeAnnotation::Int,
                        Value::Float(_) => TypeAnnotation::Float,
                        Value::String(_) => TypeAnnotation::String,
                        Value::Bool(_) => TypeAnnotation::Bool,
                        Value::Char(_) => TypeAnnotation::Char,
                        _ => TypeAnnotation::Null,
                    })
                    .unwrap_or(TypeAnnotation::Null);
                let arr_type = TypeAnnotation::Array(Box::new(inner));
                self.insert_value(name.clone(), Value::Values(items), arr_type, statement.span)?;
            }

            StatementKind::ConstantDeclaration { name, value, .. } => {
                let val = self.evaluate(value)?;
                let inferred_type = match &val {
                    Value::Integer(_) => TypeAnnotation::Int,
                    Value::Float(_) => TypeAnnotation::Float,
                    Value::String(_) => TypeAnnotation::String,
                    Value::Bool(_) => TypeAnnotation::Bool,
                    Value::Char(_) => TypeAnnotation::Char,
                    Value::Values(items) => {
                        let inner = items
                            .first()
                            .map(|v| match v {
                                Value::Integer(_) => TypeAnnotation::Int,
                                Value::Float(_) => TypeAnnotation::Float,
                                Value::String(_) => TypeAnnotation::String,
                                Value::Bool(_) => TypeAnnotation::Bool,
                                Value::Char(_) => TypeAnnotation::Char,
                                _ => TypeAnnotation::Null,
                            })
                            .unwrap_or(TypeAnnotation::Null);
                        TypeAnnotation::Array(Box::new(inner))
                    }
                    Value::Null => TypeAnnotation::Null,
                    Value::Function { .. } => TypeAnnotation::Fn,
                };

                self.insert_const(name.clone(), val, inferred_type, statement.span)?;
            }

            StatementKind::ConstantArray { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                let inner = items
                    .first()
                    .map(|v| match v {
                        Value::Integer(_) => TypeAnnotation::Int,
                        Value::Float(_) => TypeAnnotation::Float,
                        Value::String(_) => TypeAnnotation::String,
                        Value::Bool(_) => TypeAnnotation::Bool,
                        Value::Char(_) => TypeAnnotation::Char,
                        _ => TypeAnnotation::Null,
                    })
                    .unwrap_or(TypeAnnotation::Null);
                let arr_type = TypeAnnotation::Array(Box::new(inner));
                self.insert_const(name.clone(), Value::Values(items), arr_type, statement.span)?;
            }
            StatementKind::Expression(expr) => {
                self.evaluate(expr)?;
            }
            StatementKind::While { condition, body } => loop {
                let v = self.evaluate(condition)?;
                match v {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    other => {
                        return Err(self
                            .err("while condition must be a bool", statement.span)
                            .with_label(
                                condition.span,
                                format!("this is {}, expected bool", other.type_name()),
                            ));
                    }
                }
                self.evaluate_block(body)?;
            },
            StatementKind::Range(..) => {}
            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.evaluate_statement(initializer)?;
                loop {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {}
                        Value::Bool(false) => break,
                        other => {
                            return Err(self
                                .err("for condition must be a bool", statement.span)
                                .with_label(
                                    condition.span,
                                    format!("this is {}, expected bool", other.type_name()),
                                ));
                        }
                    }
                    self.evaluate_block(body)?;
                    self.evaluate(increment)?;
                }
                self.pop_scope();
            }
            StatementKind::Import { names, path } => {
                // imports are resolved at parse time; nothing to evaluate
                // or thats what i though
                // forgot that the file is removed after pr ;-;
                let mut module = &self.root_module;
                for seg in path {
                    module = module.submodules.get(seg).expect("import: unknown module");
                }
                let fns: Vec<_> = names
                    .iter()
                    .map(|name| {
                        let f = module
                            .functions
                            .get(name)
                            .unwrap_or_else(|| panic!("import: unknown function '{}'", name));
                        (name.clone(), Arc::clone(f))
                    })
                    .collect();
                for (name, f) in fns {
                    self.root_module.functions.insert(name, f);
                }
            }
            StatementKind::ForRange { .. } => {
                return Ok(()); // for now
            }
            StatementKind::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {}
                        Value::Bool(false) => return Ok(()),
                        other => {
                            return Err(self
                                .err("condition must be a bool", statement.span)
                                .with_label(
                                    condition.span,
                                    format!("this is {}, expected bool", other.type_name()),
                                ));
                        }
                    }
                    self.evaluate_block(body)?;
                }
                _ => {
                    self.evaluate_block(body)?;
                }
            },
            StatementKind::Conditional {
                if_branch,
                elseif_branch,
                else_branch,
            } => {
                if !self.evaluate_branch(if_branch)? {
                    let mut taken = false;

                    if let Some(branches) = elseif_branch {
                        for branch in branches {
                            if self.evaluate_branch(branch)? {
                                taken = true;
                                break;
                            };
                        }
                    }
                    if !taken && let Some(branch) = else_branch {
                        self.evaluate_branch(branch)?;
                    }
                }
            }

            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
            } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    return_type: Some(return_type.clone()),
                };
                self.insert_value(
                    name.clone(),
                    func.clone(),
                    TypeAnnotation::Fn,
                    statement.span,
                )?;
            }

            StatementKind::Return(expr) => {
                let value = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Null,
                };

                self.return_value = Some(value);
            }
        }
        Ok(())
    }

    fn evaluate_branch(&mut self, statement: &Statement) -> Result<bool, Error> {
        match &statement.kind {
            StatementKind::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    let v = self.evaluate(condition)?;
                    match v {
                        Value::Bool(true) => {
                            self.evaluate_block(body)?;
                            Ok(true)
                        }
                        Value::Bool(false) => Ok(false),
                        other => Err(self
                            .err("condition must be a bool", statement.span)
                            .with_label(
                                condition.span,
                                format!("this is {}, expected bool", other.type_name()),
                            )),
                    }
                }
                None => {
                    self.evaluate_block(body)?;
                    Ok(true)
                }
            },
            _ => Err(self.err("expected conditional branch", statement.span)),
        }
    }

    pub fn evaluate_block(&mut self, statements: &[Statement]) -> Result<(), Error> {
        self.push_scope();
        for statement in statements {
            self.evaluate_statement(statement)?;
            if self.return_value.is_some() {
                break;
            }
        }
        self.pop_scope();
        Ok(())
    }
}
