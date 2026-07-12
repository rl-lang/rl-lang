use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_is_error(_: &mut Evaluator, value: Value) -> bool {
    matches!(value, Value::Error(_))
}

pub fn std_error_unwrap(_: &mut Evaluator, value: Value) -> Value {
    match value {
        Value::Error(inner) => vok!(*inner),
        other => verr!(vs!(format!(
            "error_unwrap: expected error, got {}",
            other.type_name()
        ))),
    }
}
