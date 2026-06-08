use std::sync::Arc;

use crate::{
    ast::statements::{Statement, StatementKind},
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn evaluate_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.kind {
            StatementKind::VariableDeclaration { name, value, .. } => {
                let val = self.evaluate(value)?;
                let inferred_type = Evaluator::infer_type(&val);
                self.insert_value(name.clone(), val, inferred_type, statement.span)?;
            }

            StatementKind::Array { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                let arr_type = Evaluator::infer_type(&Value::Values(items.clone()));
                self.insert_value(name.clone(), Value::Values(items), arr_type, statement.span)?;
            }

            StatementKind::ConstantDeclaration { name, value, .. } => {
                let val = self.evaluate(value)?;
                let inferred_type = Evaluator::infer_type(&val);
                self.insert_const(name.clone(), val, inferred_type, statement.span)?;
            }

            StatementKind::ConstantArray { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                let arr_type = Evaluator::infer_type(&Value::Values(items.clone()));
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

                if self.is_breaking {
                    self.is_breaking = false;
                    break;
                }

                if self.is_continuing {
                    self.is_continuing = false;
                }

                if self.return_value.is_some() {
                    break;
                }
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

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                        self.evaluate(increment)?;
                        continue;
                    }

                    if self.return_value.is_some() {
                        break;
                    }

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

            StatementKind::ForRange {
                variable,
                range,
                body,
            } => {
                let items = match &range.kind {
                    StatementKind::Range(items) => items.clone(),
                    _ => {
                        return Err(
                            self.err("for-range: expected a range statement", statement.span)
                        );
                    }
                };

                for item in items {
                    self.push_scope();
                    self.insert_value(
                        variable.clone(),
                        Value::Integer(item),
                        crate::ast::statements::TypeAnnotation::Int,
                        statement.span,
                    )?;
                    self.evaluate_block(body)?;
                    self.pop_scope();

                    if self.is_breaking {
                        self.is_breaking = false;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                    }

                    if self.return_value.is_some() {
                        break;
                    }
                }
            }

            StatementKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                let arr = self.evaluate(iterable)?;
                let items = match arr {
                    Value::Values(items) => items,
                    other => {
                        return Err(self
                            .err("for-each: expected an array", statement.span)
                            .with_label(
                                iterable.span,
                                format!("this is {}, expected array", other.type_name()),
                            ));
                    }
                };
                for item in items {
                    let item_type = Evaluator::infer_type(&item);
                    self.push_scope();
                    self.insert_value(variable.clone(), item, item_type, statement.span)?;

                    self.evaluate_block(body)?;
                    self.pop_scope();

                    if self.is_breaking {
                        self.is_breaking = false;
                        break;
                    }

                    if self.is_continuing {
                        self.is_continuing = false;
                    }

                    if self.return_value.is_some() {
                        break;
                    }
                }
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
                    captured_env: vec![],
                };
                self.insert_value(
                    name.clone(),
                    func,
                    crate::ast::statements::TypeAnnotation::Fn,
                    statement.span,
                )?;

                let snapshot = self.environment.clone();
                if let Some(scope) = self.environment.last_mut() {
                    if let Some(crate::interpreter::evaluator::EnvironmentItem::PItem(p)) =
                        scope.get_mut(name)
                    {
                        if let Value::Function { captured_env, .. } = &mut p.value {
                            *captured_env = snapshot;
                        }
                    }
                }
            }

            StatementKind::Return(expr) => {
                let value = match expr {
                    Some(e) => self.evaluate(e)?,
                    None => Value::Null,
                };

                self.return_value = Some(value);
            }

            StatementKind::Break => {
                self.is_breaking = true;
            }

            StatementKind::Continue => {
                self.is_continuing = true;
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
            if self.return_value.is_some() || self.is_breaking || self.is_continuing {
                break;
            }
        }
        self.pop_scope();
        Ok(())
    }
}
