use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_arr_find_index(
    evaluator: &mut Evaluator,
    array: Value,
    function: Value,
) -> Result<Value, Error> {
    let (_, items) = match array {
        Value::Values { items_type, items } => (items_type, items),

        other => {
            return Err(Error::init(
                format!(
                    "arr_find_index() accepts only arrays found {}",
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
                "arr_find_index() expected function or lambda found {}",
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
                "arr_find_index() expected function or lambda with Bool return type found {:?}",
                return_type
            ),
            None,
            None,
        ));
    }

    let span = Span { start: 0, end: 0 };

    for (i, item) in items.iter().enumerate() {
        let mapped_item = evaluator.call_value(function.clone(), vec![item.clone()], span)?;
        if let Value::Bool(true) = mapped_item {
            return Ok(Value::Integer(i as i64));
        }
    }

    Ok(Value::Integer(-1_i64))
}
