use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::{MapKey, Value},
};

pub fn std_map_remove(_: &mut Evaluator, map: Value, key: Value) -> Value {
    match map {
        Value::Map {
            key_type,
            value_type,
            entries,
        } => {
            let map_key = match MapKey::from_value(&key) {
                Some(k) => k,
                None => {
                    return verr!(vs!(format!(
                        "map_remove: cannot remove {} from a map",
                        key.type_name()
                    )));
                }
            };
            entries.borrow_mut().remove(&map_key);
            vok!(Value::Map {
                key_type,
                value_type,
                entries,
            })
        }
        other => verr!(vs!(format!(
            "map_remove: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
