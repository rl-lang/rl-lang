use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_reduce(
    evaluator: &mut Evaluator,
    array: Value,
    function: Value,
    initial: Value,
) -> Result<Value, Error> {
    let items = match array {
        Value::Values { items, .. } => items,
        other => {
            return Err(Error::init(
                format!(
                    "arr_reduce() accepts only arrays, found {}",
                    other.type_name()
                ),
                None,
                None,
            ));
        }
    };

    if !matches!(function, Value::Function { .. }) {
        return Err(Error::init(
            format!(
                "arr_reduce() expected function or lambda, found {}",
                function.type_name()
            ),
            None,
            None,
        ));
    }

    let span = Span { start: 0, end: 0 };
    let mut result = initial;

    for item in items {
        result = evaluator.call_value(function.clone(), vec![result, item], span)?;
    }

    Ok(result)
}
