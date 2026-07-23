use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_values(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map {
            value_type,
            entries,
            ..
        } => {
            let items = entries.borrow().values().cloned().collect();
            vok!(Value::Values {
                items_type: value_type,
                items,
            })
        }
        other => verr!(vs!(format!(
            "map_values: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
