use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{vby, verr, vok, vs},
    values::Value,
};

pub fn std_is_byte(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Byte(_))
}

pub fn std_to_byte(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Integer(v) => v as u8,
        Value::Byte(v) => v,
        Value::Float(v) => v as u8,
        Value::Bool(v) => {
            if v {
                1u8
            } else {
                0u8
            }
        }
        Value::Char(v) => v as u8,
        Value::String(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                match u8::from_str_radix(&s[2..], 16) {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as byte", s))),
                }
            } else {
                match s.parse::<u8>() {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as byte", s))),
                }
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as byte",
                other.type_name()
            )));
        }
    };
    vok!(vby!(result))
}
