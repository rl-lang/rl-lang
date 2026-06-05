use std::sync::Arc;

use crate::{
    ast::statements::Statement,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn evaluate_statement(&mut self, statement: &Statement) {
        match statement {
            // the evaluator time
            Statement::Import { names, path } => {
                for name in names {
                    // the actual fun is in native.rs
                    // building the full path
                    let mut full_path = path.clone();
                    full_path.push(name.clone());

                    // resolving the full path
                    if let Some(f) = self.root_module.resolve(&full_path) {
                        // making a pointer
                        let f = Arc::clone(f);
                        // registering the import
                        self.root_module.functions.insert(name.clone(), f);
                    } else {
                        Error::init(
                            format!("'{}' not found in '{}'", name, path.join("::")),
                            None,
                            None,
                        )
                        .print_error();
                    }
                }
            }

            Statement::VariableDeclaration { name, value, .. } => {
                let val = self.evaluate(value);
                // should add type check here but for now assume the user input correctly
                self.insert_value(name.clone(), val);
            }

            Statement::Array { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    let item = self.evaluate(item);
                    items.push(item);
                }
                self.insert_value(name.clone(), Value::Values(items));
            }

            Statement::ConstantDeclaration { name, value, .. } => {
                let val = self.evaluate(value);
                // should add type check here but for now assume the user input correctly
                self.insert_const(name.clone(), val);
            }

            Statement::ConstantArray { name, value, .. } => {
                let mut items: Vec<Value> = Vec::new();
                for item in value {
                    let item = self.evaluate(item);
                    items.push(item);
                }
                self.insert_const(name.clone(), Value::Values(items));
            }

            Statement::Expression(expr) => {
                self.evaluate(expr);
            }
            Statement::While { condition, body } => loop {
                match self.evaluate(condition) {
                    Value::Bool(true) => {}
                    Value::Bool(false) => break,
                    _ => {
                        panic!();
                    }
                }
                self.evaluate_block(body);
            },
            Statement::Range(..) => {}
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.evaluate_statement(initializer);
                loop {
                    match self.evaluate(condition) {
                        Value::Bool(true) => {}
                        Value::Bool(false) => break,
                        _ => {
                            panic!();
                        }
                    }
                    self.evaluate_block(body);
                    self.evaluate(increment);
                }
            }
            Statement::ForRange { .. } => {
                // for now
                // let mut items_range: Vec<i64> = match **range {
                // Statement::Range(r) => r,
                // Statement::IterableRange(iterable_range) => {
                //    let mut items = Vec::new();
                //    for item in iterable_range {
                //        match self.evaluate(item) {
                //            Value::Float(f) => items.push(f),
                //           Value::Integer(i) => items.push(i),
                //            Value::String(s) => items.push(s),
                //            Value::Bool(b) => items.push(b),
                //            Value::Char(c) => items.push(c),
                //            Value::Null => items.push(None),
                //            _ => unreachable!(),
                //        };
                //    }
                //    items
                //}
                // _ => {
                //    Error::init("only ranges are supported for now".to_string(), None, None)
                //        .print_error();
                //    unreachable!()
                //    }
                //};
                //for item in items_range {
                //    self.evaluate_block(body);
                //}
            }
            Statement::ConditionalBranch { condition, body } => match condition {
                Some(condition) => {
                    match self.evaluate(condition) {
                        Value::Bool(true) => {}
                        Value::Bool(false) => {
                            return;
                        }
                        _ => {
                            panic!();
                        }
                    }
                    self.evaluate_block(body);
                }
                _ => {
                    self.evaluate_block(body);
                }
            },
            Statement::Conditional {
                if_branch,
                elseif_branch,
                else_branch,
            } => {
                if !self.evaluate_branch(if_branch) {
                    // weather branch of the branches condition is
                    // true and excuted or not
                    let mut taken = false;

                    if let Some(branches) = elseif_branch {
                        for branch in branches {
                            if self.evaluate_branch(branch) {
                                taken = true;
                                break;
                            };
                        }
                    }
                    if !taken && let Some(branch) = else_branch {
                        self.evaluate_branch(branch);
                    }
                }
            }
        }
    }

    fn evaluate_branch(&mut self, statement: &Statement) -> bool {
        match statement {
            Statement::ConditionalBranch { condition, body } => match condition {
                Some(condition) => match self.evaluate(condition) {
                    Value::Bool(true) => {
                        self.evaluate_block(body);
                        true
                    }
                    Value::Bool(false) => false,
                    _ => panic!(),
                },
                None => {
                    self.evaluate_block(body);
                    true
                }
            },
            _ => panic!(),
        }
    }

    pub fn evaluate_block(&mut self, statements: &[Statement]) {
        for statement in statements {
            self.evaluate_statement(statement);
        }
    }
}
