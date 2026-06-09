use crate::{interpreter::evaluator::Evaluator, interpreter::values::Value, utils::errors::Error};

pub fn std_pow(_: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    if args.len() != 2 {
        return Err(Error::init(
            format!("pow() expects 2 arguments, got {}", args.len()),
            None,
            None,
        ));
    }
    let mut iter = args.into_iter();
    let base = iter.next().unwrap();
    let exponent = iter.next().unwrap();

    match (base, exponent) {
        (Value::Integer(a), Value::Integer(b)) => {
            let b = b as u32;
            Ok(Value::Integer(a.pow(b)))
        }
        (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f64).powf(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
        (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powi(b as i32))),
        _ => Err(Error::init(
            "pow() expects numeric arguments".to_string(),
            None,
            None,
        )),
    }
}
