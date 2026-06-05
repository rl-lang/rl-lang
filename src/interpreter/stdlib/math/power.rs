use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_pow(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        return Value::Null;
    }
    match (args[0].clone(), args[1].clone()) {
        (Value::Integer(a), Value::Integer(b)) => {
            let a = a as i32;
            let b = b as u32;
            let c = a.pow(b) as i64;
            Value::Integer(c)
        }
        (Value::Integer(a), Value::Float(b)) => {
            let a = a as f64;
            let c = a.powf(b);
            Value::Float(c)
        }

        (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
        (Value::Float(a), Value::Integer(b)) => Value::Float(a.powi(b as i32)),
        _ => {
            Error::init(
                "only integers and floats are supported".to_string(),
                None,
                None,
            )
            .print_error();
            unreachable!()
        }
    }
}
