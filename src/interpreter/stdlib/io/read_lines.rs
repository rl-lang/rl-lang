use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_read_lines(_: &mut Evaluator, file: String) -> Result<Value, Error> {
    let data = std::fs::read_to_string(&file)
        .map_err(|e| {
            Error::init(
                format!("read_lines(): failed to read \"{}\": {}", file, e),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
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
