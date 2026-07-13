use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_insert(_: &mut Evaluator, array: Value, value: Value, index: i64) -> Value {
    match array {
        Value::Values { items_type, items } => {
            if index as usize >= items.len() {
                return verr!(vs!(format!("arr_insert: index out of bounds: {}", index)));
            }

            let val_type = Evaluator::infer_type(&value, false);
            if val_type != items_type && val_type != TypeAnnotation::Null {
                return verr!(vs!(format!(
                    "arr_insert: type mismatch: array expects {:?}, cannot push {:?}",
                    items_type, val_type
                )));
            }
            let mut v = items;
            v.insert(index as usize, value);
            vok!(Value::Values {
                items_type,
                items: v
            })
        }
        other => verr!(vs!(format!(
            "arr_insert: accepts only arrays and values, found {}",
            other.type_name()
        ))),
    }
}
