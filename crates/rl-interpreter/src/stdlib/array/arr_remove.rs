use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_remove(_: &mut Evaluator, array: Value, index: i64) -> Value {
    match array {
        Value::Values { items, items_type } => {
            if index as usize >= items.len() {
                return verr!(vs!(format!("arr_remove: index out of bounds: {}", index)));
            }
            let mut v = items;
            v.remove(index as usize);
            vok!(Value::Values {
                items_type,
                items: v
            })
        }
        other => verr!(vs!(format!(
            "arr_remove: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
