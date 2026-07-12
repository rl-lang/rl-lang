use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;

pub fn std_chars(_: &mut Evaluator, string: String) -> Value {
    let chars = string.chars().map(Value::Char).collect();
    Value::Values {
        items_type: TypeAnnotation::Char,
        items: chars,
    }
}
