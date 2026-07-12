use crate::{
    interpreter::evaluator::Evaluator,
    interpreter::stdlib::common::{verr, vi, vok, vs},
    interpreter::values::Value,
};

pub fn func(eval: &mut Evaluator, min: i64, max: i64) -> Value {
    if min >= max {
        return verr!(vs!(
            "min value shouldn't be bigger than or equal to maximum value".to_string()
        ));
    }

    vok!(vi!(eval.rng.generate_random_int_range(min, max)))
}
