use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vi, vok, vs},
    values::Value,
};

pub fn std_is_int(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Integer(_)) || matches!(value, Value::Byte(_))
}

pub fn std_to_int(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Integer(v) => v,
        Value::Byte(v) => v as i64,
        Value::Float(v) => v as i64,
        Value::Bool(v) => {
            if v {
                1
            } else {
                0
            }
        }
        Value::Char(v) => v as i64,
        Value::String(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                match i64::from_str_radix(&s[2..], 16) {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as int", s))),
                }
            } else {
                match s.parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as int", s))),
                }
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as int",
                other.type_name()
            )));
        }
    };
    vok!(vi!(result))
}
