use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::{MapKey, Value},
};

pub fn std_map_contains(_: &mut Evaluator, map: Value, key: Value) -> Value {
    match map {
        Value::Map { entries, .. } => {
            let map_key = match MapKey::from_value(&key) {
                Some(k) => k,
                None => return vok!(Value::Bool(false)),
            };
            vok!(Value::Bool(entries.borrow().contains_key(&map_key)))
        }
        other => verr!(vs!(format!(
            "map_contains: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
