use crate::stdlib::common::{try_fn, verr, vnl, vok, vs};
use crate::{evaluator::Evaluator, values::Value};
use std::io::{Write, stdout};

pub fn func(_: &mut Evaluator) -> Value {
    try_fn!("term_flush", stdout().flush());

    vok!(vnl!())
}
