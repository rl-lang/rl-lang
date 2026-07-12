use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_eprint(eval: &mut Evaluator, string: String, span: Span) -> Result<Value, Error> {
    Err(eval.err(string, span))
}
