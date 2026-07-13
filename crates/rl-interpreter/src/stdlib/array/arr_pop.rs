use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_pop(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => {
            if items.is_empty() {
                return verr!(vs!("arr_pop: called on empty array".to_string()));
            }
            items.pop();
            vok!(Value::Values { items_type, items })
        }
        other => verr!(vs!(format!(
            "arr_pop: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
