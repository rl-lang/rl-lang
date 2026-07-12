use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;

pub fn std_split(_: &mut Evaluator, string: String, delim: String) -> Value {
    Value::Values {
        items_type: TypeAnnotation::String,
        items: string
            .split(&delim)
            .map(|s| Value::String(s.to_string()))
            .collect(),
    }
}
