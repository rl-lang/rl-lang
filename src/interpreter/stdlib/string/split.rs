use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_split(_: &mut Evaluator, string: String, delim: String) -> Value {
    Value::Values(
        string
            .split(&delim)
            .map(|s| Value::String(s.to_string()))
            .collect(),
    )
}
