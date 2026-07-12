use crate::{evaluator::Evaluator, values::Value};
use rl_ast::statements::TypeAnnotation;
use rl_utils::{errors::Error, span::Span};

pub fn std_read_lines(eval: &mut Evaluator, file: String, span: Span) -> Result<Value, Error> {
    let data = std::fs::read_to_string(&file)
        .map_err(|e| {
            eval.err(
                format!("read_lines(): failed to read \"{}\": {}", file, e),
                span,
            )
        })?
        .lines()
        .map(String::from)
        .map(Value::String)
        .collect::<Vec<Value>>();

    Ok(Value::Values {
        items_type: TypeAnnotation::String,
        items: data,
    })
}
