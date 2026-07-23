use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_is_empty(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map { entries, .. } => vok!(Value::Bool(entries.borrow().is_empty())),
        other => verr!(vs!(format!(
            "map_is_empty: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
