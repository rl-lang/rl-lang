use std::collections::HashMap;
use std::sync::Arc;

use crate::{
    ast::nodes::Expression,
    interpreter::{
        native::{IntoNativeFn, Module},
        stdlib,
        values::Value,
    },
    utils::errors::Error,
};

pub struct Evaluator {
    pub environment: HashMap<String, (Value, bool)>,
    pub root_module: Module,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn evaluate(&mut self, expression: &Expression) -> Value {
        match expression {
            Expression::Integer(i) => Value::Integer(*i),
            Expression::String(s) => Value::String(s.clone()),
            Expression::Bool(b) => Value::Bool(*b),
            Expression::Float(f) => Value::Float(*f),
            Expression::Character(c) => Value::Char(*c),
            Expression::Index { target, index } => {
                let arr = self.evaluate(target);
                let idx = self.evaluate(index);
                match (arr, idx) {
                    (Value::Values(items), Value::Integer(i)) => {
                        let i = i as usize;
                        if i >= items.len() {
                            Error::init(
                                format!("index {} out of bounds (len {})", i, items.len()),
                                None,
                                None,
                            )
                            .print_error();
                            unreachable!()
                        }
                        items[i].clone()
                    }
                    _ => {
                        Error::init("invalid index operation".to_string(), None, None)
                            .print_error();
                        unreachable!()
                    }
                }
            }
            Expression::ArrayLiteral(items) => {
                let values = items.iter().map(|e| self.evaluate(e)).collect();
                Value::Values(values)
            }
            Expression::IndexAssign {
                target,
                index,
                value,
            } => self.index_assign(target, index, value),
            Expression::Grouping(inner) => self.evaluate(inner),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);
                self.match_binary_operator(left, right, operator)
            }
            Expression::Unary { operator, operand } => {
                let operand = self.evaluate(operand);
                self.match_unary_operator(operand, operator)
            }

            Expression::Identifier(name) => self.get_value(name.clone()),
            Expression::Assign { name, value } => {
                let val = self.evaluate(value);
                self.insert_value(name.clone(), val.clone());
                val
            }
            Expression::Call { path, args } => {
                let evaluated_args = args.iter().map(|arg| self.evaluate(arg)).collect();
                self.call_path(path, evaluated_args)
            }
        }
    }

    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            root_module: Module::new(""),
        }
    }

    pub fn with_module(mut self, m: Module) -> Self {
        self.root_module.submodules.insert(m.name.clone(), m);
        self
    }

    pub fn with_function<F, A>(mut self, name: impl Into<String>, f: F) -> Self
    where
        F: IntoNativeFn<A>,
    {
        self.root_module
            .functions
            .insert(name.into(), f.into_native());
        self
    }

    pub fn with_stdlib(self) -> Self {
        self.with_module(
            Module::new("std")
                .with_module(stdlib::math::module())
                .with_module(stdlib::display::module())
                .with_module(stdlib::io::module()),
        )
    }

    pub fn call_path(&mut self, path: &[String], args: Vec<Value>) -> Value {
        if let Some(f) = self.root_module.resolve(path) {
            let f = Arc::clone(f);
            return f(self, args);
        }
        Error::init(
            format!("undefined function {}", path.join("::")),
            None,
            None,
        )
        .print_error();
        unreachable!()
    }

    pub fn get_value(&self, value_name: String) -> Value {
        // println!("target: {}", value_name.clone());
        match self.environment.get(&value_name) {
            Some((val, _)) => val.clone(),
            None => {
                Error::init(format!("undefined variable {}", &value_name), None, None)
                    .print_error();
                unreachable!();
            }
        }
    }

    pub fn insert_value(&mut self, value_name: String, value: Value) {
        if let Some((_, true)) = self.environment.get(&value_name) {
            Error::init(
                format!("cannot assign to constant '{}'", value_name),
                None,
                None,
            )
            .print_error();
            unreachable!();
        }
        self.environment.insert(value_name, (value, false));
    }

    pub fn insert_const(&mut self, value_name: String, value: Value) {
        if self.environment.contains_key(&value_name) {
            Error::init(format!("'{}' is already declared", value_name), None, None).print_error();
            unreachable!();
        }
        self.environment.insert(value_name, (value, true));
    }
}
