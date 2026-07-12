use crate::{
    interpreter::{evaluator::Evaluator, stdlib::debug::common::assert_cmp, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    assert_cmp(eval, args, span, "assert_lt", |a, b| a < b)
}
