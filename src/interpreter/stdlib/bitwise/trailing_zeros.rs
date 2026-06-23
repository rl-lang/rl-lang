use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

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

    match args.into_iter().next().unwrap() {
        Value::Integer(x) => Ok(Value::Integer(x.trailing_zeros() as i64)),
        Value::Byte(x) => Ok(Value::Integer(x.trailing_zeros() as i64)),
        _ => Err(eval.err("trailing_zeros() expects an int or byte".to_string(), span)),
    }
}
