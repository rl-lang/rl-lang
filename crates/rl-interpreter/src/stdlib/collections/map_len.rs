use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_map_len(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map { entries, .. } => vok!(Value::Integer(entries.borrow().len() as i64)),
        other => verr!(vs!(format!(
            "map_len: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
