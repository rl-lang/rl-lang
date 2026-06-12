use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_reverse(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let mut items = items;
            items.reverse();
            Ok(Value::Values { items_type, items })
        }
        _ => Err(Error::init(
            "arr_reverse() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
