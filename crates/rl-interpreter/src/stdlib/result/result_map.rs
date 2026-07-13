use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_utils::{errors::Error, span::Span};

pub fn std_result_map(
    eval: &mut Evaluator,
    a: Value,
    b: Value,
    span: Span,
) -> Result<Value, Error> {
    match a {
        Value::Ok(inner) => {
            let mapped = match eval.call_value(b, vec![*inner], span) {
                Ok(mapped) => mapped,
                Err(e) => return Ok(verr!(vs!(e.message().to_string()))),
            };
            Ok(vok!(mapped))
        }
        // pass error as is
        Value::Err(_) => Ok(a),
        other => Ok(verr!(vs!(format!(
            "result_map: expected result, got {}",
            other.type_name()
        )))),
    }
}

pub fn std_result_map_err(
    eval: &mut Evaluator,
    a: Value,
    b: Value,
    span: Span,
) -> Result<Value, Error> {
    match a {
        Value::Err(inner) => {
            let mapped = match eval.call_value(b, vec![*inner], span) {
                Ok(mapped) => mapped,
                Err(e) => return Ok(verr!(vs!(e.message().to_string()))),
            };
            Ok(verr!(mapped))
        }
        // pass ok as is
        Value::Ok(_) => Ok(a),
        other => Ok(verr!(vs!(format!(
            "result_map_err: expected result, got {}",
            other.type_name()
        )))),
    }
}
