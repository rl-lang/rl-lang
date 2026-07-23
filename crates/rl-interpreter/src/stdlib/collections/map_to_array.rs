use std::rc::Rc;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_map_to_array(_: &mut Evaluator, map: Value) -> Value {
    match map {
        Value::Map {
            key_type,
            value_type,
            entries,
        } => {
            let items = entries
                .borrow()
                .iter()
                .map(|(k, v)| Value::Tuple(vec![k.clone().into_value(), v.clone()]))
                .collect();
            vok!(Value::Values {
                items_type: TypeAnnotation::Tuple(Rc::new(vec![key_type, value_type])),
                items,
            })
        }
        other => verr!(vs!(format!(
            "map_to_array: accepts only maps, found {}",
            other.type_name()
        ))),
    }
}
