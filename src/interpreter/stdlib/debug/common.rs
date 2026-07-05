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

pub fn assert_cmp(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
    name: &str,
    op: fn(f64, f64) -> bool,
) -> Result<Value, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(eval.err(
            format!("{}() expects 2 or 3 arguments, got {}", name, args.len()),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    let (fa, fb) = match (as_f64(a), as_f64(b)) {
        (Some(fa), Some(fb)) => (fa, fb),
        _ => {
            return Err(eval.err(
                format!(
                    "{}: expects numeric arguments, got {} and {}",
                    name,
                    a.type_name(),
                    b.type_name()
                ),
                span,
            ));
        }
    };

    if !op(fa, fb) {
        let default_msg = format!("{} failed: `{}` vs `{}`", name, a, b);
        let message = match args.get(2) {
            Some(Value::String(s)) => format!("{}: {}", s, default_msg),
            Some(other) => {
                return Err(eval.err(
                    format!(
                        "{}: expects a string message, got {}",
                        name,
                        other.type_name()
                    ),
                    span,
                ));
            }
            None => default_msg,
        };
        return Err(eval.err(message, span));
    }

    Ok(Value::Null)
}
