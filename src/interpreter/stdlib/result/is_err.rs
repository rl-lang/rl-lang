use crate::{
    interpreter::{evaluator::Evaluator, stdlib::common::check_arity, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(_: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "is_err", span)?;
    Ok(Value::Bool(args[0].is_err()))
}
