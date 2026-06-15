use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_arr_min(_: &mut Evaluator, array: Value) -> Result<Value, Error> {
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
                    .ok_or_else(|| {
                        Error::init(
                            "arr_min() called on empty array".to_string(),
                            None,
                            Some(ErrorReason::init(Reason::Runtime, None)),
                        )
                    })?;
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
                    .ok_or_else(|| {
                        Error::init(
                            "arr_min() called on empty array".to_string(),
                            None,
                            Some(ErrorReason::init(Reason::Runtime, None)),
                        )
                    })?;
                Ok(Value::Float(min))
            }
            _ => Err(Error::init(
                "arr_min() accepts only int or float arrays".to_string(),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )),
        },
        _ => Err(Error::init(
            "arr_min() accepts only arrays".to_string(),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )),
    }
}
