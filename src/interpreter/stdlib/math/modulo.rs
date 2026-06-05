use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_mod(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        return Value::Null;
    }
    match (&args[0], &args[1]) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(a % b),
        (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
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
