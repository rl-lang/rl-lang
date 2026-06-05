use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_tan(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        return Value::Null;
    }
    match args[0] {
        Value::Integer(i) => Value::Float((i as f64).tan()),
        Value::Float(fl) => Value::Float(fl.tan()),
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
