use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_arr_push(_: &mut Evaluator, array: Value, value: Value) -> Value {
    match array {
        Value::Values { items_type, items } => {
            let val_type = Evaluator::infer_type(&value, false);
            if !Evaluator::types_compatible(&val_type, &items_type) {
                return verr!(vs!(format!(
                    "arr_push: type mismatch: array expects {:?}, cannot push {:?}",
                    items_type, val_type
                )));
            }
            let mut v = items;
            v.push(value);
            vok!(Value::Values {
                items_type,
                items: v
            })
        }
        other => verr!(vs!(format!(
            "arr_push: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
