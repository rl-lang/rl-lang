use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_flat_map(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (items_type, items) = match array {
        Value::Values { items_type, items } => (items_type, items),
        other => {
            return Ok(verr!(vs!(format!(
                "arr_flat_map: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    let Value::Function(data) = &function else {
        return Ok(verr!(vs!(format!(
            "arr_flat_map: expected function or lambda, found {}",
            function.type_name()
        ))));
    };

    if !matches!(data.return_type, Some(TypeAnnotation::Array(_))) {
        return Ok(verr!(vs!(format!(
            "arr_flat_map: expected function or lambda with Array return type, found {:?}",
            data.return_type
        ))));
    }

    let mut result = Vec::with_capacity(items.len());

    for item in items {
        let mapped_item = eval.call_value(function.clone(), vec![item], span)?;
        if let Value::Values { items, .. } = mapped_item {
            result.extend(items);
        }
    }

    Ok(vok!(Value::Values {
        items_type,
        items: result
    }))
}
