use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_exit(_: &mut Evaluator, code: i64, _: Span) -> Result<Value, Error> {
    std::process::exit(code as i32);
}
