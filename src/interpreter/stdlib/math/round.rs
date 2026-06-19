use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_round(eval: &mut Evaluator, a: Value, span: Span) -> Result<Value, Error> {
    match a {
        Value::Integer(i) => Ok(Value::Integer(i)),
        Value::Float(f) => Ok(Value::Float(f.round())),
        other => Err(eval.err(
            format!("round() expects a number, got {}", other.type_name(),),
            span,
        )),
    }
}
