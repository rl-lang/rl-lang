use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_unique(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let mut seen = Vec::new();
            for item in items {
                if !seen.contains(&item) {
                    seen.push(item);
                }
            }
            Ok(Value::Values {
                items_type,
                items: seen,
            })
        }
        _ => Err(Error::init(
            "arr_unique() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
