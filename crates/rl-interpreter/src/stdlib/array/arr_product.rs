use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_product(_: &mut Evaluator, array: Value) -> Value {
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
                vok!(Value::Integer(product))
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
                    .product::<f64>();
                vok!(Value::Float(product))
            }
            _ => verr!(vs!(
                "arr_product: accepts only int or float arrays".to_string()
            )),
        },
        other => verr!(vs!(format!(
            "arr_product: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
