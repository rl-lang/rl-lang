use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_bit_not(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.err(
            format!("bit_not() expects 1 argument, got {}", args.len()),
            span,
        ));
    }

    match args.into_iter().next().unwrap_or(Value::Null) {
        Value::Byte(x) => Ok(Value::Byte(!x)),
        Value::Integer(x) => Ok(Value::Integer(!x)),
        _ => Err(eval.err("bit_not() expects a byte or an int".to_string(), span)),
    }
}
