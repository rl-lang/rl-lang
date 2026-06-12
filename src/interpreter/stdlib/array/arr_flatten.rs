use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_flatten(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => {
            if items.is_empty() {
                return Err(Error::init(
                    "arr_flatten() called on empty array".to_string(),
                    None,
                    None,
                ));
            }

            Ok(Value::Values {
                items_type: items_type,
                items: items
                    .into_iter()
                    .flat_map(|v| {
                        if let Value::Values { items, .. } = v {
                            items
                        } else {
                            vec![v]
                        }
                    })
                    .collect(),
            })
        }
        _ => Err(Error::init(
            "arr_flatten() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
