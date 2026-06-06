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
                self.insert_value(name.clone(), val, statement.span)?;
            }

            StatementKind::Array { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                self.insert_value(name.clone(), Value::Values(items), statement.span)?;
            }

            StatementKind::ConstantDeclaration { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.insert_const(name.clone(), val, statement.span)?;
            }

            StatementKind::ConstantArray { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    items.push(self.evaluate(item)?);
                }
                self.insert_const(name.clone(), Value::Values(items), statement.span)?;
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
        for statement in statements {
            self.evaluate_statement(statement)?;
        }
        Ok(())
    }
}
