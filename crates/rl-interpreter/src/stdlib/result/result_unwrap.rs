use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vs},
    values::Value,
};

pub fn std_unwrap(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Ok(inner) => *inner,
        Value::Err(v) => verr!(vs!(format!("result_unwrap: called on Err({})", v))),
        other => verr!(vs!(format!(
            "result_unwrap: expected result, got {}",
            other.type_name()
        ))),
    }
}

pub fn std_unwrap_err(_: &mut Evaluator, v: Value) -> Value {
    match v {
        Value::Err(inner) => *inner,
        Value::Ok(v) => verr!(vs!(format!("result_unwrap_err: called on ok({})", v))),
        other => verr!(vs!(format!(
            "result_unwrap_err: expected result, got {}",
            other.type_name()
        ))),
    }
}

pub fn std_unwrap_or(_: &mut Evaluator, v: Value, b: Value) -> Value {
    match v {
        Value::Ok(inner) => *inner,
        Value::Err(_) => b,
        other => verr!(vs!(format!(
            "result_unwrap_or: expected result, got {}",
            other.type_name()
        ))),
    }
}
