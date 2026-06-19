use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{
        errors::{Error, ErrorReason, Reason},
        span::Span,
    },
};

pub fn std_arr_max(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => match items_type {
            TypeAnnotation::Int => {
                let max = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Integer(i) = v {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .max()
                    .ok_or_else(|| eval.err("arr_max() called on empty array".to_string(), span))?;
                Ok(Value::Integer(max))
            }
            TypeAnnotation::Float => {
                let max = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Float(f) = v {
                            Some(f)
                        } else {
                            None
                        }
                    })
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .ok_or_else(|| eval.err("arr_max() called on empty array".to_string(), span))?;
                Ok(Value::Float(max))
            }
            _ => Err(eval.err(
                "arr_max() accepts only int or float arrays".to_string(),
                span,
            )),
        },
        _ => Err(eval.err("arr_max() accepts only arrays".to_string(), span)),
    }
}
