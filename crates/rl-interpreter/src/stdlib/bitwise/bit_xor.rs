use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_bit_xor(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.err(
            format!("bit_xor() expects 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let mut iter = args.into_iter();
    let a = iter.next().unwrap_or(Value::Null);
    let b = iter.next().unwrap_or(Value::Null);

    match (a, b) {
        (Value::Byte(x), Value::Byte(y)) => Ok(Value::Byte(x ^ y)),
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x ^ y)),
        _ => Err(eval.err(
            "bit_xor() expects (byte, byte) or (int, int) arguments".to_string(),
            span,
        )),
    }
}
