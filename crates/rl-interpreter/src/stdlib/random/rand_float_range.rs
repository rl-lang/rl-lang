use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vf, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, min: f64, max: f64) -> Value {
    if min >= max {
        return verr!(vs!(
            "min value shouldn't be bigger than or equal to maximum value".to_string()
        ));
    }

    vok!(vf!(eval.rng.generate_random_float_range(min, max)))
}
