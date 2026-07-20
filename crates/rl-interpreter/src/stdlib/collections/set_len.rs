use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_set_len(_: &mut Evaluator, set: Value) -> Value {
    match set {
        Value::Set { items, .. } => vok!(Value::Integer(items.borrow().len() as i64)),
        other => verr!(vs!(format!(
            "set_len: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
