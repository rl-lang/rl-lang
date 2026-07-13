use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(eval.err(
            format!("assert: expects 1 or 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let cond = match &args[0] {
        Value::Bool(b) => *b,
        other => {
            return Err(eval.err(
                format!(
                    "assert: expects a bool condition, got {}",
                    other.type_name()
                ),
                span,
            ));
        }
    };

    if !cond {
        let message = match args.get(1) {
            Some(Value::String(s)) => s.clone(),
            Some(other) => {
                return Err(eval.err(
                    format!(
                        "assert: expects a string message, got {}",
                        other.type_name()
                    ),
                    span,
                ));
            }
            None => "assertion failed".to_string(),
        };
        return Err(eval.err(message, span));
    }

    Ok(Value::Null)
}
