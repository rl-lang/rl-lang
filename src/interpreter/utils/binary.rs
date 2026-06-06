use crate::{
    interpreter::evaluator::Evaluator,
    interpreter::values::Value,
    lexer::tokentypes::TokenType,
    utils::{errors::Error, span::Span},
};

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
                _ => {
                    return Err(self.type_mismatch_binary(
                        "+", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Minus => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "-", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Star => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "*", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Slash => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
                (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "/", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Less => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a < b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "<", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Greater => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a > b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        ">", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::LessEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a <= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "<=", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a >= b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        ">=", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::BangEqual => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a != b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a != b),
                (Value::String(a), Value::String(b)) => Value::Bool(a != b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a != b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a != b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "!=", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            TokenType::Compare => match (&left, &right) {
                (Value::Integer(a), Value::Integer(b)) => Value::Bool(a == b),
                (Value::Float(a), Value::Float(b)) => Value::Bool(a == b),
                (Value::String(a), Value::String(b)) => Value::Bool(a == b),
                (Value::Char(a), Value::Char(b)) => Value::Bool(a == b),
                (Value::Bool(a), Value::Bool(b)) => Value::Bool(a == b),
                _ => {
                    return Err(self.type_mismatch_binary(
                        "==", &left, left_span, &right, right_span, span,
                    ))
                }
            },
            _ => return Err(self.err("unknown binary operator", span)),
        };
        Ok(v)
    }
}
