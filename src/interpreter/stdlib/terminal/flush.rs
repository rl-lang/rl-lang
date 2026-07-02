use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use std::io::{Write, stdout};

pub fn func(_: &mut Evaluator) -> Value {
    stdout()
        .flush()
        .map_err(|e| verr!(vs!(format!("term_flush(): {}", e))));

    vok!(vnl!())
}
