use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_first(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, .. } => match items.into_iter().next() {
            Some(v) => vok!(v),
            None => verr!(vs!("arr_first: called on empty array".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_first: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
