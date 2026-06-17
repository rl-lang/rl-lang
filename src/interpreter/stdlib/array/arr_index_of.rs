use std::ops::Index;

use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_index_of(_: &mut Evaluator, array: Value, index: i64) -> Result<Value, Error> {
    match array {
        Value::Values { items, .. } => {
            if index as usize >= items.len() {
                return Err(Error::init(
                    format!("index out of bounds: {}", index),
                    None,
                    None,
                ));
            }
            let item = items.index(index as usize);
            Ok(item.clone())
        }
        _ => Err(Error::init(
            "arr_is_empty() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
