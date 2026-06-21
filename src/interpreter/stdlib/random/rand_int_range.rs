use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, min: i64, max: i64, span: Span) -> Result<Value, Error> {
    if min >= max {
        return Err(eval.err(
            "min value shouldn't be bigger than or equal to maximum value",
            span,
        ));
    }

    Ok(Value::Integer(eval.rng.generate_random_int_range(min, max)))
}
