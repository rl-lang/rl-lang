use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_clear(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map {
            key_type,
            value_type,
            entries,
        } => {
            entries.borrow_mut().clear();
            vok!(Value::Map {
                key_type,
                value_type,
                entries,
            })
        }
        other => verr!(vs!(format!(
            "map_clear: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
