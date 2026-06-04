use std::collections::HashMap;

use crate::{
    ast::nodes::Expression,
    interpreter::{stdlib, values::Value},
    utils::errors::Error,
};

pub struct Evaluator {
    pub environment: HashMap<String, Value>,
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
