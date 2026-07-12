use crate::{
    interpreter::{
        evaluator::Evaluator,
        stdlib::common::{verr, vf, vok, vs},
        values::Value,
    },
    utils::span::Span,
};

pub fn func(eval: &mut Evaluator, function: Value, iterations_val: Value) -> Value {
    if !matches!(function, Value::Function { .. }) {
        return verr!(vs!(format!(
            "bench: expects a function or lambda, got {}",
            function.type_name()
        )));
    }

    let iterations = match iterations_val {
        Value::Integer(n) if n > 0 => n as u64,
        other => {
            return verr!(vs!(format!(
                "bench: expects a positive int for iterations, got {}",
                other.type_name()
            )));
        }
    };

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        if let Err(e) = eval.call_value(function.clone(), vec![], Span::dummy()) {
            return verr!(vs!(format!(
                "bench: error executing the function: {}",
                e.message()
            )));
        };
    }
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    vok!(vf!(elapsed_ms))
}
