use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;

pub fn std_args(eval: &mut Evaluator) -> Value {
    let args: Vec<Value> = std::env::args()
        .skip(eval.user_args_offset)
        .map(Value::String)
        .collect();
    Value::Values {
        items_type: TypeAnnotation::String,
        items: args,
    }
}
