use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_pop(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => {
            if items.is_empty() {
                return Err(Error::init(
                    "pop() called on empty array".to_string(),
                    None,
                    None,
                ));
            }
            items.pop();
            Ok(Value::Values { items_type, items })
        }
        _ => Err(Error::init(
            "pop() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
