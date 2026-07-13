use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

pub fn std_arr_any(
    eval: &mut Evaluator,
    array: Value,
    function: Value,
    span: Span,
) -> Result<Value, Error> {
    let items = match array {
        Value::Values { items, .. } => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_any: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    let Value::Function(data) = &function else {
        return Ok(verr!(vs!(format!(
            "arr_any: expected function or lambda, found {}",
            function.type_name()
        ))));
    };

    if !matches!(data.return_type, Some(TypeAnnotation::Bool)) {
        return Ok(verr!(vs!(format!(
            "arr_any: expected function or lambda with Bool return type, found {:?}",
            data.return_type
        ))));
    }

    for item in items {
        let mapped_item = eval.call_value(function.clone(), vec![item], span)?;
        if let Value::Bool(true) = mapped_item {
            return Ok(vok!(Value::Bool(true)));
        }
    }

    Ok(vok!(Value::Bool(false)))
}
