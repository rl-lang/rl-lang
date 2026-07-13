use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_is_empty(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, .. } => vok!(Value::Bool(items.is_empty())),
        other => verr!(vs!(format!(
            "arr_is_empty: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
