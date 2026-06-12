use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
};

pub fn std_arr_range(_: &mut Evaluator, start: i64, end: i64, step: i64) -> Value {
    Value::Values {
        items_type: TypeAnnotation::Int,
        items: (start..end)
            .step_by(step as usize)
            .map(|v| Value::Integer(v))
            .collect(),
    }
}
