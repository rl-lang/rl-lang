use crate::{evaluator::Evaluator, stdlib::debug::common::assert_cmp, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    assert_cmp(eval, args, span, "assert_gt", |a, b| a > b)
}
