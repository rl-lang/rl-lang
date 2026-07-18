use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_set_to_array(_: &mut Evaluator, set: Value) -> Value {
    match set {
        Value::Set {
            items_type,
            items,
        } => {
            let items = items
                .borrow()
                .iter()
                .map(|k| k.clone().into_value())
                .collect();
            vok!(Value::Values {
                items_type,
                items,
            })
        }
        other => verr!(vs!(format!(
            "set_to_array: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
