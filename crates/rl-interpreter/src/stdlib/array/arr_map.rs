use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_map(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (items_type, items) = match array {
        Value::Values { items_type, items } => (items_type, items),
        other => {
            return Ok(verr!(vs!(format!(
                "arr_map: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(function, Value::Function { .. }) {
        return Ok(verr!(vs!(format!(
            "arr_map: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    let mut result = Vec::with_capacity(items.len());

    for item in items {
        let mapped_item = eval.call_value(function.clone(), vec![item], span)?;
        result.push(mapped_item);
    }

    let item_type = result
        .first()
        .map(|first| Evaluator::infer_type(first, false))
        .unwrap_or(items_type);

    Ok(vok!(Value::Values {
        items_type: item_type,
        items: result
    }))
}
