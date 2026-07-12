use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_find(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (_, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(eval.err(
                format!("arr_find() accepts only arrays found {}", other.type_name()).to_string(),
                span,
            ));
        }
    };
    let Value::Function(data) = &function else {
        return Err(eval.err(
            format!(
                "arr_find() expected function or lambda found {}",
                function.type_name()
            ),
            span,
        ));
    };

    if !matches!(data.return_type, Some(TypeAnnotation::Bool)) {
        return Err(eval.err(
            format!(
                "arr_find() expected function or lambda with Bool return type found {:?}",
                data.return_type
            ),
            span,
        ));
    }

    for item in items {
        let mapped_item = eval.call_value(function.clone(), vec![item.clone()], span)?;
        if let Value::Bool(true) = mapped_item {
            return Ok(item);
        }
    }

    Ok(Value::Null)
}
