use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_count(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, .. } => vok!(Value::Integer(items.len() as i64)),
        other => verr!(vs!(format!(
            "arr_count: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
