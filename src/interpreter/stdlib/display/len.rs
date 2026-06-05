use crate::{interpreter::values::Value, utils::errors::Error};

pub fn std_len(args: Vec<Value>) -> Value {
    match &args[0] {
        Value::Values(items) => Value::Integer(items.len() as i64),
        Value::String(s) => Value::Integer(s.len() as i64),
        _ => {
            Error::init("len() expects an array or string".to_string(), None, None).print_error();
            unreachable!()
        }
    }
}
