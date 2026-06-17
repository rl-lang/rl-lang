use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_arr_product(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => match items_type {
            TypeAnnotation::Int => {
                let product = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Integer(i) = v {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .product::<i64>();
                Ok(Value::Integer(product))
            }
            TypeAnnotation::Float => {
                let product = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Float(f) = v {
                            Some(f)
                        } else {
                            None
                        }
                    })
                    .sum::<f64>();
                Ok(Value::Float(product))
            }
            _ => Err(Error::init(
                "arr_product() accepts only int or float arrays".to_string(),
                None,
                None,
            )),
        },
        _ => Err(Error::init(
            "arr_product() accepts only arrays".to_string(),
            None,
            None,
        )),
    }
}
