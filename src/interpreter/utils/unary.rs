use crate::{
    interpreter::evaluator::Evaluator,
    interpreter::values::Value,
    lexer::tokentypes::TokenType,
    utils::{errors::Error, span::Span},
};

impl Evaluator {
    fn type_mismatch_unary(
        &self,
        op: &str,
        operand: &Value,
        operand_span: Span,
        span: Span,
    ) -> Error {
        self.err(format!("type mismatch on {}", op), span)
            .with_label(operand_span, format!("this is {}", operand.type_name()))
    }

    pub fn match_unary_operator(
        &mut self,
        operand: Value,
        operand_span: Span,
        operator: &TokenType,
        span: Span,
    ) -> Result<Value, Error> {
        let v = match operator {
            TokenType::Bang => match &operand {
                Value::Bool(b) => Value::Bool(!b),
                _ => return Err(self.type_mismatch_unary("!", &operand, operand_span, span)),
            },
            TokenType::Minus => match &operand {
                Value::Integer(i) => Value::Integer(-i),
                Value::Float(f) => Value::Float(-f),
                _ => return Err(self.type_mismatch_unary("-", &operand, operand_span, span)),
            },
            _ => return Err(self.err("unknown unary operator", span)),
        };
        Ok(v)
    }
}
