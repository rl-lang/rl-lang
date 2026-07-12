use crate::{
    interpreter::{evaluator::Evaluator, stdlib::debug::common::as_f64, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    // third spot IS NOT MESSAGE it is epsilon
    if args.len() < 2 || args.len() > 3 {
        return Err(eval.err(
            format!(
                "assert_approx_eq: expects 2 or 3 arguments, got {}",
                args.len()
            ),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    let (fa, fb) = match (as_f64(a), as_f64(b)) {
        (Some(fa), Some(fb)) => (fa, fb),
        _ => {
            return Err(eval.err(
                format!(
                    "assert_approx_eq: expects numeric arguments, got {} and {}",
                    a.type_name(),
                    b.type_name()
                ),
                span,
            ));
        }
    };

    let epsilon = match args.get(2) {
        Some(v) => as_f64(v).ok_or_else(|| {
            eval.err(
                format!(
                    "assert_approx_eq: expects a numeric epsilon, got {}",
                    v.type_name()
                ),
                span,
            )
        })?,
        None => 1e-9,
    };

    if (fa - fb).abs() > epsilon {
        return Err(eval.err(
            format!(
                "assert_approx_eq failed: `{}` and `{}` differ by more than {}",
                a, b, epsilon
            ),
            span,
        ));
    }

    Ok(Value::Null)
}
