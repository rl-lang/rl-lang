use crate::{
    interpreter::evaluator::Evaluator, interpreter::values::Value, lexer::tokentypes::TokenType,
    utils::errors::Error,
};

impl Evaluator {
    pub fn match_binary_operator(
        &mut self,
        left: Value,
        right: Value,
        operator: &TokenType,
    ) -> Value {
        match operator {
            TokenType::Plus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                _ => {
                    Error::init("type mismatch on +".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Minus => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                _ => {
                    Error::init("type mismatch on -".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Star => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                _ => {
                    Error::init("type mismatch on *".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Slash => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => {
                    Error::init("type mismatch on /".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Less => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a < b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                _ => {
                    Error::init("type mismatch on <".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Greater => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a > b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                _ => {
                    Error::init("type mismatch on >".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a <= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                _ => {
                    Error::init("type mismatch on <=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a >= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                _ => {
                    Error::init("type mismatch on >=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::BangEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a != b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a != b),
                (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a != b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),

                _ => {
                    Error::init("type mismatch on !=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::Compare => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a == b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
                (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                _ => {
                    Error::init("type mismatch on ==".to_string(), None, None).print_error();
                    unreachable!();
                }
            },

            TokenType::PlusEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Integer(a), Value::Float(b)) => Value::Integer(a + b as i64),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                _ => {
                    Error::init("type mismatch on +=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },
            TokenType::MinusEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Integer(a), Value::Float(b)) => Value::Integer(a - b as i64),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a - b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                _ => {
                    Error::init("type mismatch on -=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },

            TokenType::StarEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Integer(a), Value::Float(b)) => Value::Integer(a * b as i64),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a * b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                _ => {
                    Error::init("type mismatch on *=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },

            TokenType::SlashEqual => match (left, right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                (Value::Integer(a), Value::Float(b)) => Value::Integer(a / b as i64),
                (Value::Float(a), Value::Integer(b)) => Value::Float(a / b as f64),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => {
                    Error::init("type mismatch on /=".to_string(), None, None).print_error();
                    unreachable!();
                }
            },

            _ => {
                Error::init("some error".to_string(), None, None).print_error();
                unreachable!();
            }
        }
    }
}
