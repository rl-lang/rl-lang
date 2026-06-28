use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn std_is_float(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Float(_))
}

pub fn std_to_float(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Float(f) => f,
        Value::Integer(i) => i as f64,
        Value::Byte(i) => i as f64,
        Value::Bool(b) => {
            if b {
                1.0
            } else {
                0.0
            }
        }
        Value::String(s) => match s.trim().parse::<f64>() {
            Ok(f) => f,
            Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as float", s))),
        },

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as float",
                other.type_name()
            )));
        }
    };

    vok!(vf!(result))
}
