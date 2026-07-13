use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_max(_: &mut Evaluator, array: Value) -> Value {
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
                .max()
            {
                Some(max) => vok!(Value::Integer(max)),
                None => verr!(vs!("arr_max: called on empty array".to_string())),
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
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            {
                Some(max) => vok!(Value::Float(max)),
                None => verr!(vs!("arr_max: called on empty array".to_string())),
            },
            _ => verr!(vs!("arr_max: accepts only int or float arrays".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_max: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
