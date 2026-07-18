use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_flatten(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, items_type } => vok!(Value::Values {
            items_type,
            items: items
                .into_iter()
                .flat_map(|v| {
                    if let Value::Values { items, .. } = v {
                        items
                    } else {
                        vec![v]
                    }
                })
                .collect(),
        }),
        other => verr!(vs!(format!(
            "arr_flatten: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
