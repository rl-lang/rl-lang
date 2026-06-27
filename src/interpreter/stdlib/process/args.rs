use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_args(eval: &mut Evaluator, _: Vec<Value>, _: Span) -> Result<Value, Error> {
    let args: Vec<Value> = std::env::args()
        .skip(eval.user_args_offset) // skip the interpreter binary itself
        .map(Value::String)
        .collect();
    Ok(Value::Values {
        items_type: TypeAnnotation::String,
        items: args,
    })
}
