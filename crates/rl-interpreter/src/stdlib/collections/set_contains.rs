use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::{MapKey, Value},
};

pub fn std_set_contains(_: &mut Evaluator, set: Value, value: Value) -> Value {
    match set {
        Value::Set { items, .. } => {
            let key = match MapKey::from_value(&value) {
                Some(k) => k,
                None => return vok!(Value::Bool(false)),
            };
            vok!(Value::Bool(items.borrow().contains(&key)))
        }
        other => verr!(vs!(format!(
            "set_contains: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
