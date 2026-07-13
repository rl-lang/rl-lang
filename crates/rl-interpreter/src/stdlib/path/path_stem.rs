use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_stem(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => match std::path::Path::new(&s).file_stem() {
            Some(stem) => vok!(vs!(stem.to_string_lossy().to_string())),
            None => verr!(vs!(format!("path_stem: \"{}\" has no file stem", s))),
        },
        other => verr!(vs!(format!(
            "path_stem: expects a string, got {}",
            other.type_name()
        ))),
    }
}
