use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_trailing_zeros(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.err(
            format!("trailing_zeros() expects 1 argument, got {}", args.len()),
            span,
        ));
    }

    match args.into_iter().next().unwrap_or(Value::Null) {
        Value::Byte(x) => Ok(Value::Byte(u8::trailing_zeros(x) as u8)),
        Value::Integer(x) => Ok(Value::Integer(i64::trailing_zeros(x) as i64)),
        _ => Err(eval.err(
            "trailing_zeros() expects a byte or an int".to_string(),
            span,
        )),
    }
}
