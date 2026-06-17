use crate::{
    ast::statements::TypeAnnotation,
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_list_dir(_: &mut Evaluator, path: String) -> Result<Value, Error> {
    let data = std::fs::read_dir(&path)
        .map_err(|e| {
            Error::init(
                format!("list_dir(): failed to read \"{}\": {}", path, e),
                None,
                Some(ErrorReason::init(Reason::Runtime, None)),
            )
        })?
        .filter_map(|i| i.ok())
        .map(|i| i.path().to_string_lossy().to_string())
        .map(Value::String)
        .collect::<Vec<Value>>();

    Ok(Value::Values {
        items_type: TypeAnnotation::String,
        items: data,
    })
}
