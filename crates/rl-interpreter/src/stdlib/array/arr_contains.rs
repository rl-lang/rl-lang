use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_contains(_: &mut Evaluator, array: Value, value: Value) -> Value {
    match array {
        Value::Values { items, .. } => vok!(Value::Bool(items.contains(&value))),
        other => verr!(vs!(format!(
            "arr_contains: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
