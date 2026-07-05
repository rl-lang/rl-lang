use crate::{
    interpreter::{
        evaluator::Evaluator,
        stdlib::common::{verr, vok, vs},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn as_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Integer(i) => Some(*i as f64),
        Value::Float(f) => Some(*f),
        Value::Byte(b) => Some(*b as f64),
        _ => None,
    }
}

pub fn assert_eq_message(
    a: &Value,
    b: &Value,
    custom: Option<&Value>,
    name: &str,
    expected_equal: bool,
) -> Value {
    let op = if expected_equal { "!=" } else { "==" };
    let default_msg = format!(
        "{} failed: left `{}` ({}) {} right `{}` ({})",
        name,
        a,
        a.type_name(),
        op,
        b,
        b.type_name()
    );

    match custom {
        Some(Value::String(s)) => vok!(vs!(format!("{}: {}", s, default_msg))),
        Some(other) => verr!(vs!(format!(
            "{}() expects a string message, got {}",
            name,
            other.type_name()
        ))),
        None => vok!(vs!(default_msg)),
    }
}

