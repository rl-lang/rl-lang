use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_sort(_: &mut Evaluator, array: Value) -> Value {
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
                vok!(Value::Values { items_type, items })
            }
            TypeAnnotation::Float => {
                items.sort_by(|a, b| {
                    if let (Value::Float(x), Value::Float(y)) = (a, b) {
                        x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                vok!(Value::Values { items_type, items })
            }
            _ => verr!(vs!("arr_sort: accepts only int or float arrays".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_sort: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
