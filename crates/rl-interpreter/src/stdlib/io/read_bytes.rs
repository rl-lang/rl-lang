use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn func(eval: &mut Evaluator, file: String, span: Span) -> Result<Value, Error> {
    let data = std::fs::read(&file)
        .map_err(|e| {
            eval.err(
                format!("read_bytes(): failed to read \"{}\": {}", file, e),
                span,
            )
        })?
        .into_iter()
        .map(Value::Byte)
        .collect::<Vec<Value>>();

    Ok(Value::Values {
        items_type: TypeAnnotation::Byte,
        items: data,
    })
}
