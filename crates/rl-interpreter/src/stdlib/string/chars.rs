use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
};

pub fn std_chars(_: &mut Evaluator, string: String) -> Value {
    let chars = string.chars().map(Value::Char).collect();
    Value::Values {
        items_type: TypeAnnotation::Char,
        items: chars,
    }
}
