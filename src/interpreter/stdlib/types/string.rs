use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_is_string(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::String(_))
}

pub fn std_to_string(_: &mut Evaluator, value: Value) -> Value {
    let result = match value {
        Value::Integer(v) => format!("{}", v),
        Value::Byte(v) => format!("{}", v),
        Value::Float(v) => format!("{}", v),
        Value::Bool(v) => format!("{}", v),
        Value::Char(v) => v.to_string(),
        Value::String(s) => s,

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as string",
                other.type_name()
            )));
        }
    };
    vok!(vs!(result))
}
