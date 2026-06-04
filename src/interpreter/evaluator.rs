use std::collections::HashMap;

use crate::{
    ast::nodes::Expression,
    interpreter::{stdlib, values::Value},
    utils::errors::Error,
};

pub struct Evaluator {
    environment: HashMap<String, Value>,
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

            Expression::IndexAssign {
                target,
                index,
                value,
            } => {
                let idx = self.evaluate(index);
                let val = self.evaluate(value);

                // get the root array name
                fn get_root_name(expression: &Expression) -> &str {
                    match expression {
                        Expression::Identifier(array_name) => array_name,
                        Expression::Index { target, .. } => get_root_name(target),
                        _ => unreachable!(),
                    }
                }

                fn get_indices_as_vec(
                    expression: &Expression,
                    evaluator: &mut Evaluator,
                ) -> Vec<usize> {
                    match expression {
                        Expression::Identifier(_) => vec![],
                        Expression::Index { target, index } => {
                            let mut indices = get_indices_as_vec(target, evaluator);
                            if let Value::Integer(i) = evaluator.evaluate(index) {
                                indices.push(i as usize);
                            }
                            indices
                        }
                        _ => unreachable!(),
                    }
                }

                let root = get_root_name(&target).to_string();
                let mut indices = get_indices_as_vec(&target, self);
                if let Value::Integer(i) = idx {
                    indices.push(i as usize);
                }

                let root_array = self.environment.get_mut(&root).unwrap();
                let mut current_array = root_array;

                for i in &indices[..indices.len() - 1] {
                    if let Value::Values(items) = current_array {
                        current_array = &mut items[*i];
                    }
                }
                if let Value::Values(items) = current_array {
                    items[*indices.last().unwrap()] = val.clone();
                }

                val
            }
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
            Expression::Call { name, args } => {
                let evaluated_args = args.iter().map(|arg| self.evaluate(arg)).collect();
                self.call_function(name, evaluated_args)
            }
        }
    }

    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn get_value(&self, value_name: String) -> Value {
        // println!("target: {}", value_name.clone());
        match self.environment.get(&value_name) {
            Some(val) => val.clone(),
            _ => {
                Error::init(format!("undefined variable {}", &value_name), None, None)
                    .print_error();
                unreachable!();
            }
        }
    }

    pub fn insert_value(&mut self, value_name: String, value: Value) {
        self.environment.insert(value_name.clone(), value.clone());
        // println!("{}, {:?}", value_name.clone(), value.clone());
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> Value {
        if stdlib::display::is_in_display(&name) {
            stdlib::display::match_std_display(name, args)
        } else if stdlib::math::is_in_math(&name) {
            stdlib::math::match_std_math(name, args)
        } else if stdlib::io::is_in_io(&name) {
            stdlib::io::match_std_io(name, args)
        } else {
            Error::init(format!("undefined function {}", name), None, None).print_error();
            unreachable!();
        }
    }
}
