use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_sort(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values {
            items_type,
            mut items,
        } => match items_type {
            TypeAnnotation::Int => {
                items.sort_by(|a, b| {
                    if let (Value::Integer(x), Value::Integer(y)) = (a, b) {
                        x.cmp(y)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                Ok(Value::Values { items_type, items })
            }
            TypeAnnotation::Float => {
                items.sort_by(|a, b| {
                    if let (Value::Float(x), Value::Float(y)) = (a, b) {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                Ok(Value::Values { items_type, items })
            }
            _ => Err(Error::init(
                "arr_sort() accepts only int or float arrays".to_string(),
                None,
                None,
            )),
        },
        _ => Err(Error::init(
            "arr_sort() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
