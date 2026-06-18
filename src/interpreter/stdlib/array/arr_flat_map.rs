use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_flat_map(
    evaluator: &mut Evaluator,
    array: Value,
    function: Value,
) -> Result<Value, Error> {
    let (items_type, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(Error::init(
                format!(
                    "arr_flat_map() accepts only arrays found {}",
                    other.type_name()
                )
                .to_string(),
                None,
                None,
            ));
        }
    };
    if !matches!(function, Value::Function { .. }) {
        return Err(Error::init(
            format!(
                "arr_flat_map() expected function or lambda found {}",
                function.type_name()
            ),
            None,
            None,
        ));
    }

    match function.clone() {
        Value::Function { return_type, .. } => {
            if !matches!(return_type, Some(TypeAnnotation::Array(_))) {
                return Err(Error::init(
                    format!(
                        "arr_flat_map() expected function or lambda with Array return type found {:?}",
                        return_type
                    ),
                    None,
                    None,
                ));
            }
        }
        _ => {}
    }

    let span = Span { start: 0, end: 0 };

    let mut result = Vec::with_capacity(items.len());

    for item in items {
        let mapped_item = evaluator.call_value(function.clone(), vec![item.clone()], span)?;
        match mapped_item {
            Value::Values { items, .. } => {
                for item in items {
                    result.push(item)
                }
            }
            _ => {}
        }
    }

    Ok(Value::Values {
        items_type,
        items: result,
    })
}
