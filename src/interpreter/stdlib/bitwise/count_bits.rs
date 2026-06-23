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

    match args.into_iter().next().unwrap() {
        Value::Integer(x) => Ok(Value::Integer(x.count_ones() as i64)),
        Value::Byte(x) => Ok(Value::Integer(x.count_ones() as i64)),
        _ => Err(eval.err("count_bits() expects an int or byte".to_string(), span)),
    }
}
