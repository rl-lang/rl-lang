use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() > 1 {
        return Err(eval.err(
            format!("unreachable: expects 0 or 1 arguments, got {}", args.len()),
            span,
        ));
    }

    let message = match args.into_iter().next() {
        Some(Value::String(s)) => format!("internal error: entered unreachable code: {}", s),
        Some(other) => {
            return Err(eval.err(
                format!(
                    "unreachable: expects a string message, got {}",
                    other.type_name()
                ),
                span,
            ));
        }
        None => "internal error: entered unreachable code".to_string(),
    };

    Err(eval.err(message, span))
}
