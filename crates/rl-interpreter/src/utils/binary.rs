//! Binary operator evaluation for all supported operand type combinations.
//!
//! All type mismatches emit a labeled error pointing at both operands.

use crate::{evaluator::Evaluator, values::Value};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Evaluator {
    fn type_mismatch_binary(
        &self,
        op: &str,
        left: &Value,
        left_span: Span,
        right: &Value,
        right_span: Span,
        span: Span,
    ) -> Error {
        self.err(format!("type mismatch on {}", op), span)
            .with_label(left_span, format!("this is {}", left.type_name()))
            .with_label(right_span, format!("this is {}", right.type_name()))
    }

    pub fn match_binary_operator(
        &mut self,
        left: Value,
        left_span: Span,
        right: Value,
        right_span: Span,
        operator: &TokenType,
        span: Span,
    ) -> Result<Value, Error> {
        let v = match operator {
            TokenType::Plus => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                (Value::Byte(a), Value::Byte(b)) => Value::Byte(a + b),

                _ => {
                    return Err(
                        self.type_mismatch_binary("+", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Minus => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                (Value::Byte(a), Value::Byte(b)) => Value::Byte(a - b),

                _ => {
                    return Err(
                        self.type_mismatch_binary("-", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Star => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                (Value::Byte(a), Value::Byte(b)) => Value::Byte(a * b),

                _ => {
                    return Err(
                        self.type_mismatch_binary("*", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Slash => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => {
                    if *b == 0 {
                        return Err(self.err("division by zero", span));
                    }
                    Value::Integer(a / b)
                }
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                (Value::Byte(a), Value::Byte(b)) => {
                    if *b == 0 {
                        return Err(self.err("division by zero", span));
                    }
                    Value::Byte(a / b)
                }

                _ => {
                    return Err(
                        self.type_mismatch_binary("/", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Less => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a < b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a < b),
                _ => {
                    return Err(
                        self.type_mismatch_binary("<", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Greater => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a > b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a > b),
                _ => {
                    return Err(
                        self.type_mismatch_binary(">", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::LessEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a <= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a <= b),
                _ => {
                    return Err(
                        self.type_mismatch_binary("<=", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a >= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a >= b),
                _ => {
                    return Err(
                        self.type_mismatch_binary(">=", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::BangEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a != b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a != b),
                (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a != b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a != b),
                (
                    Value::Enum {
                        name: a_name,
                        variant: a_var,
                    },
                    Value::Enum {
                        name: b_name,
                        variant: b_var,
                    },
                ) => Value::Bool(a_name != b_name || a_var != b_var),
                _ => {
                    return Err(
                        self.type_mismatch_binary("!=", &left, left_span, &right, right_span, span)
                    );
                }
            },
            TokenType::Compare => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a == b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
                (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                (Value::Byte(a), Value::Byte(b)) => Value::Bool(a == b),
                (
                    Value::Enum {
                        name: a_name,
                        variant: a_var,
                    },
                    Value::Enum {
                        name: b_name,
                        variant: b_var,
                    },
                ) => Value::Bool(a_name == b_name && a_var == b_var),
                _ => {
                    return Err(
                        self.type_mismatch_binary("==", &left, left_span, &right, right_span, span)
                    );
                }
            },
            _ => return Err(self.err("unknown binary operator", span)),
        };
        Ok(v)
    }
}
