use crate::{evaluator::Evaluator, stdlib::common::check_arity, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_result_map(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 2, "result_map", span)?;
    match args[0].clone() {
        Value::Ok(inner) => {
            let mapped = eval.call_value(args[1].clone(), vec![*inner], span)?;
            Ok(Value::Ok(Box::new(mapped)))
        }
        // pass error as is
        Value::Err(_) => Ok(args[0].clone()),
        other => Err(eval.err(
            format!("result_map: expected result, got {}", other.type_name()),
            span,
        )),
    }
}

pub fn std_result_map_err(
    eval: &mut Evaluator,
    args: Vec<Value>,
    span: Span,
) -> Result<Value, Error> {
    check_arity(&args, 2, "result_map_err", span)?;
    match args[0].clone() {
        Value::Err(inner) => {
            let mapped = eval.call_value(args[1].clone(), vec![*inner], span)?;
            Ok(Value::Err(Box::new(mapped)))
        }
        // pass ok as is
        Value::Ok(_) => Ok(args[0].clone()),
        other => Err(eval.err(
            format!("result_map_err: expected result, got {}", other.type_name()),
            span,
        )),
    }
}
