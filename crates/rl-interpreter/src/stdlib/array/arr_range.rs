use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_arr_range(_: &mut Evaluator, start: i64, end: i64, step: i64) -> Value {
    if step <= 0 {
        return verr!(vs!(format!(
            "arr_range: step must be positive, got {}",
            step
        )));
    }

    vok!(Value::Values {
        items_type: TypeAnnotation::Int,
        items: (start..end)
            .step_by(step as usize)
            .map(Value::Integer)
            .collect(),
    })
}
