use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_slice(_: &mut Evaluator, array: Value, start: i64, end: i64) -> Value {
    match array {
        Value::Values { items_type, items } => {
            let start = start as usize;
            let end = end as usize;
            if start > items.len() || end > items.len() {
                return verr!(vs!(format!(
                    "arr_slice: index out of bounds: {}..{} (len {})",
                    start,
                    end,
                    items.len()
                )));
            }
            if start > end {
                return verr!(vs!(format!(
                    "arr_slice: start {} is greater than end {}",
                    start, end
                )));
            }
            vok!(Value::Values {
                items_type,
                items: items[start..end].to_vec(),
            })
        }
        other => verr!(vs!(format!(
            "arr_slice: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
