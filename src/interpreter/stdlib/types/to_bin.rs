use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_to_bin(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Byte(v) => format!("{:b}", v),
        Value::Integer(v) => format!("{:b}", v),
        Value::Bool(v) => {
            if v {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        Value::Char(v) => format!("{:b}", v as u32),
        Value::String(s) => s.bytes().map(|b| format!("{:b}", b)).collect::<String>(),

        other => {
            return Value::Err(Box::new(Value::String(format!(
                "cannot parse \"{}\" as binary",
                other.type_name()
            ))));
        }
    };

    Value::Ok(Box::new(Value::String(result)))
}
