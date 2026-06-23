use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_bit_and(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(eval.err(
            format!("bit_and() expects 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let mut iter = args.into_iter();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    match (a, b) {
        (Value::Byte(x), Value::Byte(y)) => Ok(Value::Byte(x & y)),
        (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x & y)),
        _ => Err(eval.err(
            "bit_and() expects byte or integer arguments".to_string(),
            span,
        )),
    }
}
