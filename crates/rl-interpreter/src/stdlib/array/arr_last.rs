use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_last(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, .. } => match items.into_iter().last() {
            Some(v) => vok!(v),
            None => verr!(vs!("arr_last: called on empty array".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_last: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
