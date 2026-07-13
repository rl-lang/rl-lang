use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_min(_: &mut Evaluator, array: Value) -> Value {
    match array {
        Value::Values { items, items_type } => match items_type {
            TypeAnnotation::Int => match items
                .into_iter()
                .filter_map(|v| {
                    if let Value::Integer(i) = v {
                        Some(i)
                    } else {
                        None
                    }
                })
                .min()
            {
                Some(min) => vok!(Value::Integer(min)),
                None => verr!(vs!("arr_min: called on empty array".to_string())),
            },
            TypeAnnotation::Float => match items
                .into_iter()
                .filter_map(|v| {
                    if let Value::Float(f) = v {
                        Some(f)
                    } else {
                        None
                    }
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            {
                Some(min) => vok!(Value::Float(min)),
                None => verr!(vs!("arr_min: called on empty array".to_string())),
            },
            _ => verr!(vs!("arr_min: accepts only int or float arrays".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_min: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
