use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_sum(_: &mut Evaluator, array: Value) -> Value {
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
                vok!(Value::Integer(sum))
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
                vok!(Value::Float(sum))
            }
            _ => verr!(vs!("arr_sum: accepts only int or float arrays".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_sum: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
