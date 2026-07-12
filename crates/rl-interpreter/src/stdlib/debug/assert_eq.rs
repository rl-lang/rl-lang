use crate::{
    interpreter::{
        evaluator::Evaluator,
        stdlib::{common::extract_string, debug::common::assert_eq_message},
        values::Value,
    },
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(eval.err(
            format!("assert_eq: expects 2 or 3 arguments, got {}", args.len()),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    if a != b {
        let err = match assert_eq_message(a, b, args.get(2), "assert_eq", true) {
            Value::Ok(k) => *k,
            Value::Err(e) => *e,
            _ => {
                unreachable!()
            }
        };
        let err_string = match extract_string(err, "assert_eq") {
            Err(a) | Ok(a) => a,
        };

        return Err(eval.err(err_string, span));
    }
    Ok(Value::Null)
}
