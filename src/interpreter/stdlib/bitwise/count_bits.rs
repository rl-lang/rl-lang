use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_count_bits(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() != 1 {
        return Err(eval.err(
            format!("count_bits() expects 1 argument, got {}", args.len()),
            span,
        ));
    }

    match args.into_iter().next().unwrap_or(Value::Null) {
        Value::Byte(x) => Ok(Value::Byte(x.count_ones() as u8)),
        Value::Integer(x) => Ok(Value::Integer(x.count_ones() as i64)),
        _ => Err(eval.err("count_bits() expects a byte or an int".to_string(), span)),
    }
}
