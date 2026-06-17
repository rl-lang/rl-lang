use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::{Error, ErrorReason, Reason},
};

pub fn std_rename_file(_: &mut Evaluator, path: String, new_name: String) -> Result<Value, Error> {
    let old_path = std::path::Path::new(&path);
    let new_path = match old_path.parent() {
        Some(parent) => parent.join(&new_name),
        None => std::path::PathBuf::from(&new_name),
    };

    std::fs::rename(old_path, &new_path).map_err(|e| {
        Error::init(
            format!(
                "rename_file(): failed to rename \"{}\" to \"{}\": {}",
                path,
                new_path.to_string_lossy(),
                e
            ),
            None,
            Some(ErrorReason::init(Reason::Runtime, None)),
        )
    })?;

    Ok(Value::String(new_path.to_string_lossy().to_string()))
}
