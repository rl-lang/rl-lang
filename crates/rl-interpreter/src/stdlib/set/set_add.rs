use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::{MapKey, Value},
};

pub fn std_set_add(_: &mut Evaluator, set: Value, value: Value) -> Value {
    match set {
        Value::Set { items_type, items } => {
            let val_type = Evaluator::infer_type(&value, false);
            if !Evaluator::types_compatible(&val_type, &items_type) {
                return verr!(vs!(format!(
                    "set_add: type mismatch: set expects {:?}, cannot add {:?}",
                    items_type, val_type
                )));
            }
            let key = match MapKey::from_value(&value) {
                Some(k) => k,
                None => {
                    return verr!(vs!(format!(
                        "set_add: cannot add {} to a set",
                        value.type_name()
                    )))
                }
            };
            items.borrow_mut().insert(key);
            vok!(Value::Set { items_type, items })
        }
        other => verr!(vs!(format!(
            "set_add: accepts only sets, found {}",
            other.type_name()
        ))),
    }
}
