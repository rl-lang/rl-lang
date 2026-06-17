use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_slice(
    _: &mut Evaluator,
    array: Value,
    start: i64,
    end: i64,
) -> Result<Value, Error> {
    match array {
        Value::Values { items_type, items } => {
            let start = start as usize;
            let end = end as usize;
            if start > items.len() || end > items.len() {
                return Err(Error::init(
                    format!(
                        "slice index out of bounds: {}..{} (len {})",
                        start,
                        end,
                        items.len()
                    ),
                    None,
                    None,
                ));
            }
            if start > end {
                return Err(Error::init(
                    format!("slice start {} is greater than end {}", start, end),
                    None,
                    None,
                ));
            }
            Ok(Value::Values {
                items_type,
                items: items[start..end].to_vec(),
            })
        }
        _ => Err(Error::init(
            "arr_slice() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
