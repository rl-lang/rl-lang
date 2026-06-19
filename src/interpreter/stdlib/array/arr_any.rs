use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_any(
    evaluator: &mut Evaluator,
    array: Value,
    function: Value,
) -> Result<Value, Error> {
    let (_, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(Error::init(
                format!("arr_any() accepts only arrays found {}", other.type_name()).to_string(),
                None,
                None,
            ));
        }
    };
    if !matches!(function, Value::Function { .. }) {
        return Err(Error::init(
            format!(
                "arr_any() expected function or lambda found {}",
                function.type_name()
            ),
            None,
            None,
        ));
    }

    if let Value::Function { return_type, .. } = function.clone()
        && !matches!(return_type, Some(TypeAnnotation::Bool))
    {
        return Err(Error::init(
            format!(
                "arr_any() expected function or lambda with Bool return type found {:?}",
                return_type
            ),
            None,
            None,
        ));
    }

    let span = Span { start: 0, end: 0 };

    for item in items.clone() {
        let mapped_item = evaluator.call_value(function.clone(), vec![item.clone()], span)?;
        if let Value::Bool(true) = mapped_item {
            return Ok(Value::Bool(true));
        }
    }

    Ok(Value::Bool(false))
}
