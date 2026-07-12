use crate::interpreter::{evaluator::Evaluator, stdlib::common::vs, values::Value};

pub fn func(eval: &mut Evaluator) -> Value {
    match &eval.source_file {
        Some(f) => vs!(f.name.to_string()),
        None => Value::Null,
    }
}
