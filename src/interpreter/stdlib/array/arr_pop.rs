use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_pop(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => {
            if items.is_empty() {
                return Err(Error::init(
                    "arr_pop() called on empty array".to_string(),
                    None,
                    None,
                ));
            }
            items.pop();
            Ok(Value::Values { items_type, items })
        }
        _ => Err(Error::init(
            "arr_pop() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
