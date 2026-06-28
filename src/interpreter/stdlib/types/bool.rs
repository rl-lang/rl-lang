use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{vb, verr, vok, vs},
    values::Value,
};

pub fn std_is_bool(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Bool(_))
}

pub fn std_to_bool(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Bool(b) => b,
        Value::Integer(i) => i != 0,
        Value::Byte(i) => i != 0,
        Value::Float(f) => f != 0.0,
        Value::Null => false,
        Value::String(s) => match s.trim() {
            "false" | "0" | "" => false,
            _ => true,
        },

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as bool",
                other.type_name()
            )));
        }
    };

    vok!(vb!(result))
}
