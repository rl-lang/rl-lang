use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_flat_map(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (items_type, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(eval.err(
                format!(
                    "arr_flat_map() accepts only arrays found {}",
                    other.type_name()
                )
                .to_string(),
                span,
            ));
        }
    };
    let Value::Function(data) = &function else {
        return Err(eval.err(
            format!(
                "arr_flat_map() expected function or lambda found {}",
                function.type_name()
            ),
            span,
        ));
    };

    if !matches!(data.return_type, Some(TypeAnnotation::Array(_))) {
        return Err(eval.err(
            format!(
                "arr_flat_map() expected function or lambda with Array return type found {:?}",
                data.return_type
            ),
            span,
        ));
    }

    let mut result = Vec::with_capacity(items.len());

    for item in items {
        let mapped_item = eval.call_value(function.clone(), vec![item.clone()], span)?;
        if let Value::Values { items, .. } = mapped_item {
            for item in items {
                result.push(item)
            }
        }
    }

    Ok(Value::Values {
        items_type,
        items: result,
    })
}
