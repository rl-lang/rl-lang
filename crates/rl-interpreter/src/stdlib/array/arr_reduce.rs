use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_reduce(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    initial: Value,
    span: Span,
) -> Result<Value, Error> {
    let items = match array {
        Value::Values { items, .. } => items,
        other => {
            return Err(eval.err(
                format!(
                    "arr_reduce() accepts only arrays, found {}",
                    other.type_name()
                ),
                span,
            ));
        }
    };

    if !matches!(function, Value::Function { .. }) {
        return Err(eval.err(
            format!(
                "arr_reduce() expected function or lambda, found {}",
                function.type_name()
            ),
            span,
        ));
    }

    let mut result = initial;

    for item in items {
        result = eval.call_value(function.clone(), vec![result, item], span)?;
    }

    Ok(result)
}
