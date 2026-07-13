use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_reverse(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => {
            items.reverse();
            vok!(Value::Values { items_type, items })
        }
        other => verr!(vs!(format!(
            "arr_reverse: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
