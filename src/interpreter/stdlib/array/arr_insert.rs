use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_insert(
    _: &mut Evaluator,
    array: Value,
    value: Value,
    index: i64,
) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            if index as usize >= items.len() {
                return Err(Error::init(
                    format!("index out of bounds: {}", index),
                    None,
                    None,
                ));
            }

            let val_type = Evaluator::infer_type(&value);
            if val_type != items_type && val_type != TypeAnnotation::Null {
                return Err(Error::init(
                    format!(
                        "type mismatch: array expects {:?}, cannot push {:?}",
                        items_type, val_type
                    ),
                    None,
                    None,
                ));
            }
            let mut v = items;
            v.insert(index as usize, value);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(Error::init(
            "arr_insert() accepts only arrays and values".to_string(),
            None,
            None,
        )),
    }
}
