use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_set_is_empty(_: &mut Evaluator, set: Value) -> Value {
    match set {
        Value::Set { items, .. } => vok!(Value::Bool(items.borrow().is_empty())),
        other => verr!(vs!(format!(
            "set_is_empty: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
