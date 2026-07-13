use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() > 1 {
        return Err(eval.err(
            format!("panic: expects 0 or 1 arguments, got {}", args.len()),
            span,
        ));
    }
    let message = match args.into_iter().next() {
        Some(Value::String(s)) => s,
        Some(other) => {
            return Err(eval.err(
                format!("panic: expects a string message, got {}", other.type_name()),
                span,
            ));
        }
        None => "explicit panic".to_string(),
    };
    Err(eval.err(message, span))
}
