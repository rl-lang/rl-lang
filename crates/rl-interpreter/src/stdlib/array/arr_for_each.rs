use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_for_each(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let items = match array {
        Value::Values { items, .. } => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_for_each: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    let Value::Function(data) = &function else {
        return Ok(verr!(vs!(format!(
            "arr_for_each: expected function or lambda, found {}",
            function.type_name()
        ))));
    };

    if !matches!(data.return_type, Some(TypeAnnotation::Null)) {
        return Ok(verr!(vs!(format!(
            "arr_for_each: expected function or lambda with no (or null) return type, found {:?}",
            data.return_type
        ))));
    }

    for item in items {
        eval.call_value(function.clone(), vec![item], span)?;
    }

    Ok(vok!(Value::Null))
}
