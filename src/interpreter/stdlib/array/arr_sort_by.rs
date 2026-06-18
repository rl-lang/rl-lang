use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_sort_by(
    evaluator: &mut Evaluator,
    array: Value,
    function: Value,
) -> Result<Value, Error> {
    let (items_type, mut items) = match array {
        Value::Values { items_type, items } => (items_type, items),
        other => {
            return Err(Error::init(
                format!(
                    "arr_sort_by() accepts only arrays, found {}",
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
                "arr_sort_by() expected function or lambda, found {}",
                function.type_name()
            ),
            None,
            None,
        ));
    }

    let span = Span { start: 0, end: 0 };

    for i in 1..items.len() {
        let mut j = i;
        while j > 0 {
            let result = evaluator.call_value(
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
                    return Err(Error::init(
                        format!(
                            "arr_sort_by() comparator must return int (-1, 0, 1), found {}",
                            other.type_name()
                        ),
                        None,
                        None,
                    ));
                }
            }
        }
    }

    Ok(Value::Values { items_type, items })
}
