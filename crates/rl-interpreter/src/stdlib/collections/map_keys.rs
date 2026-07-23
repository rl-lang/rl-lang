use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_keys(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map {
            key_type, entries, ..
        } => {
            let items = entries
                .borrow()
                .keys()
                .map(|k| k.clone().into_value())
                .collect();
            vok!(Value::Values {
                items_type: key_type,
                items,
            })
        }
        other => verr!(vs!(format!(
            "map_keys: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
