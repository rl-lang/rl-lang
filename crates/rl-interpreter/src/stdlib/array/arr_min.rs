use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_min(eval: &mut Evaluator, array: Value, span: Span) -> Result<Value, Error> {
    match array {
        Value::Values { items, items_type } => match items_type {
            TypeAnnotation::Int => {
                let min = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Integer(i) = v {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .min()
                    .ok_or_else(|| eval.err("arr_min() called on empty array".to_string(), span))?;
                Ok(Value::Integer(min))
            }
            TypeAnnotation::Float => {
                let min = items
                    .into_iter()
                    .filter_map(|v| {
                        if let Value::Float(f) = v {
                            Some(f)
                        } else {
                            None
                        }
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .ok_or_else(|| eval.err("arr_min() called on empty array".to_string(), span))?;
                Ok(Value::Float(min))
            }
            _ => Err(eval.err(
                "arr_min() accepts only int or float arrays".to_string(),
                span,
            )),
        },
        _ => Err(eval.err("arr_min() accepts only arrays".to_string(), span)),
    }
}
