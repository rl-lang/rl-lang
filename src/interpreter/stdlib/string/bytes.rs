use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
};

pub fn std_bytes(_: &mut Evaluator, string: String) -> Value {
    let bytes = string.bytes().map(Value::Byte).collect();
    Value::Values {
        items_type: TypeAnnotation::Byte,
        items: bytes,
    }
}
