use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_unique(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items_type, items } => {
            let mut seen = Vec::new();
            for item in items {
                if !seen.contains(&item) {
                    seen.push(item);
                }
            }
            vok!(Value::Values {
                items_type,
                items: seen
            })
        }
        other => verr!(vs!(format!(
            "arr_unique: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
