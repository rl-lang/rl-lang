use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, min: f64, max: f64, span: Span) -> Result<Value, Error> {
    if min >= max {
        return Err(eval.err(
            "min value shouldn't be bigger than or equal to maximum value",
            span,
        ));
    }

    Ok(Value::Float(eval.rng.generate_random_float_range(min, max)))
}
