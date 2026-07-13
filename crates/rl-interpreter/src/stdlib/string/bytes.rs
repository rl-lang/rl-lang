use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;

pub fn std_bytes(_: &mut Evaluator, string: String) -> Value {
    let bytes = string.bytes().map(Value::Byte).collect();
    Value::Values {
        items_type: TypeAnnotation::Byte,
        items: bytes,
    }
}
