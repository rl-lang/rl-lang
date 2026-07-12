use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_range(_: &mut Evaluator, start: i64, end: i64, step: i64) -> Value {
    Value::Values {
        items_type: TypeAnnotation::Int,
        items: (start..end)
            .step_by(step as usize)
            .map(Value::Integer)
            .collect(),
    }
}
