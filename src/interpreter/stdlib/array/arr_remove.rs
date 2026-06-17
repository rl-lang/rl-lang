use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_remove(_: &mut Evaluator, array: Value, index: i64) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => {
            if index as usize >= items.len() {
                return Err(Error::init(
                    format!("index out of bounds: {}", index),
                    None,
                    None,
                ));
            }
            let mut v = items;
            v.remove(index as usize);
            Ok(Value::Values {
                items_type,
                items: v,
            })
        }
        _ => Err(Error::init(
            "arr_remove() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
