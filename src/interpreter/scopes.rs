use std::collections::HashMap;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

impl Evaluator {
    pub fn push_scope(&mut self) {
        self.environment.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.environment.pop();
    }

    pub fn get_value(&self, value_name: String) -> Value {
        for scope in self.environment.iter().rev() {
            if let Some((val, _)) = scope.get(&value_name) {
                return val.clone();
            }
        }
        Error::init(format!("undefined variable {}", &value_name), None, None).print_error();
        unreachable!();
    }

    pub fn insert_value(&mut self, value_name: String, value: Value) {
        let scope = self.environment.last_mut().unwrap();
        scope.insert(value_name, (value, false));
    }

    pub fn assign_value(&mut self, value_name: String, value: Value) {
        for scope in self.environment.iter_mut().rev() {
            if let Some(entry) = scope.get_mut(&value_name) {
                if entry.1 {
                    Error::init(
                        format!("cannot assign to constant '{}'", value_name),
                        None,
                        None,
                    )
                    .print_error();
                    unreachable!();
                }
                entry.0 = value;
                return;
            }
        }
        Error::init(format!("undefined variable '{}'", value_name), None, None).print_error();
        unreachable!();
    }
    pub fn insert_const(&mut self, value_name: String, value: Value) {
        let scope = self.environment.last_mut().unwrap();
        if scope.contains_key(&value_name) {
            Error::init(format!("'{}' is already declared", value_name), None, None).print_error();
            unreachable!();
        }
        scope.insert(value_name, (value, true));
    }
}
