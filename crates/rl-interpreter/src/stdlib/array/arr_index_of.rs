use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_index_of(_: &mut Evaluator, array: Value, value: Value) -> Value {
    match array {
        Value::Values { items, .. } => match items.iter().position(|item| *item == value) {
            Some(pos) => vok!(Value::Integer(pos as i64)),
            None => vok!(Value::Integer(-1)),
        },
        other => verr!(vs!(format!(
            "arr_index_of: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
