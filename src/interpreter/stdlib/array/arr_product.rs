use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_product(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
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
            _ => Err(eval.err(
                "arr_product() accepts only int or float arrays".to_string(),
                span,
            )),
        },
        _ => Err(eval.err("arr_product() accepts only arrays".to_string(), span)),
    }
}
