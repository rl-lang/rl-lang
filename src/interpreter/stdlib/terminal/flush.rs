use crate::interpreter::stdlib::common::{verr, vnl, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use std::io::{Write, stdout};

pub fn func(_: &mut Evaluator) -> Value {
    if let Err(e) = stdout().flush() {
        return verr!(vs!(format!("term_flush(): {}", e)));
    };

    vok!(vnl!())
}
