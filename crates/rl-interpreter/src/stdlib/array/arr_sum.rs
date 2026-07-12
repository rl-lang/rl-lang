use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_sum(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => match items_type {
            TypeAnnotation::Int => {
                let sum = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Integer(i) = v {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();
                Ok(Value::Integer(sum))
            }
            TypeAnnotation::Float => {
                let sum = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Float(f) = v {
                            Some(f)
                        } else {
                            None
                        }
                    })
                    .sum::<f64>();
                Ok(Value::Float(sum))
            }
            _ => Err(eval.err(
                "arr_sum() accepts only int or float arrays".to_string(),
                span,
            )),
        },
        _ => Err(eval.err("arr_sum() accepts only arrays".to_string(), span)),
    }
}
