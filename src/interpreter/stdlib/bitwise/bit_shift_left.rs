use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_bit_shift_left(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.err(
            format!("bit_shift_left() expects 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let mut iter = args.into_iter();
    let a = iter.next().unwrap();
    let shift = iter.next().unwrap();

    match (a, shift) {
        (Value::Integer(x), Value::Integer(s)) => Ok(Value::Integer(x << (s as u32))),
        (Value::Byte(x), Value::Integer(s)) => Ok(Value::Byte(x << (s as u32))),
        _ => Err(eval.err("bit_shift_left() expects (int|byte, int)".to_string(), span)),
    }
}
