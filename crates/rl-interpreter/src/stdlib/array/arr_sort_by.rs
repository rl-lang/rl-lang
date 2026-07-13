use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_sort_by(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (items_type, mut items) = match array {
        Value::Values { items_type, items } => (items_type, items),
        other => {
            return Ok(verr!(vs!(format!(
                "arr_sort_by: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };

    if !matches!(function, Value::Function { .. }) {
        return Ok(verr!(vs!(format!(
            "arr_sort_by: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    for i in 1..items.len() {
        let mut j = i;
        while j > 0 {
            let result = eval.call_value(
                function.clone(),
                vec![items[j - 1].clone(), items[j].clone()],
                span,
            )?;

            match result {
                Value::Integer(n) if n > 0 => {
                    items.swap(j - 1, j);
                    j -= 1;
                }
                Value::Integer(_) => break,
                other => {
                    return Ok(verr!(vs!(format!(
                        "arr_sort_by: comparator must return int (-1, 0, 1), found {}",
                        other.type_name()
                    ))));
                }
            }
        }
    }

    Ok(vok!(Value::Values { items_type, items }))
}
