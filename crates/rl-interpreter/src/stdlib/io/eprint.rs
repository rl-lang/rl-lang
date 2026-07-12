use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;
use crate::utils::span::Span;

pub fn std_eprint(eval: &mut Evaluator, string: String, span: Span) -> Result<Value, Error> {
    Err(eval.err(string, span))
}
