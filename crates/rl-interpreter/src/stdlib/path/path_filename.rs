use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_filename(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => match std::path::Path::new(&s).file_name() {
            Some(name) => vok!(vs!(name.to_string_lossy().to_string())),
            None => verr!(vs!(format!(
                "path_filename: \"{}\" has no file name component",
                s
            ))),
        },
        other => verr!(vs!(format!(
            "path_filename: expects a string, got {}",
            other.type_name()
        ))),
    }
}
