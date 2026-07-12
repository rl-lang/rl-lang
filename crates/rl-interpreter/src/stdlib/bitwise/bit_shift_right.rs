use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_bit_shift_right(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.err(
            format!("bit_shift_right() expects 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let mut iter = args.into_iter();
    let a = iter.next().unwrap_or(Value::Null);
    let shift = iter.next().unwrap_or(Value::Null);

    match (a, shift) {
        (Value::Byte(x), Value::Byte(s)) => Ok(Value::Byte(x >> (s as u32))),
        (Value::Byte(x), Value::Integer(s)) => Ok(Value::Byte(x >> (s as u32))),
        (Value::Integer(x), Value::Byte(s)) => Ok(Value::Integer(x >> (s as u32))),
        (Value::Integer(x), Value::Integer(s)) => Ok(Value::Integer(x >> (s as u32))),
        _ => Err(eval.err(
            "bit_shift_right() expects ((byte|int), (int|byte))".to_string(),
            span,
        )),
    }
}
