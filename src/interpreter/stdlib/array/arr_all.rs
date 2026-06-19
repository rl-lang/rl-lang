use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_all(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let (_, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(eval.err(
                format!("arr_all() accepts only arrays found {}", other.type_name()).to_string(),
                span,
            ));
        }
    };
    if !matches!(function, Value::Function { .. }) {
        return Err(eval.err(
            format!(
                "arr_all() expected function or lambda found {}",
                function.type_name()
            ),
            span,
        ));
    }

    if let Value::Function { return_type, .. } = function.clone()
        && !matches!(return_type, Some(TypeAnnotation::Bool))
    {
        return Err(eval.err(
            format!(
                "arr_all() expected function or lambda with Bool return type found {:?}",
                return_type
            ),
            span,
        ));
    }

    for item in items.clone() {
        let mapped_item = eval.call_value(function.clone(), vec![item.clone()], span)?;
        if let Value::Bool(false) = mapped_item {
            return Ok(Value::Bool(false));
        }
    }

    Ok(Value::Bool(true))
}
