use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_extension(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => match std::path::Path::new(&s).extension() {
            Some(ext) => vok!(vs!(ext.to_string_lossy().to_string())),
            None => verr!(vs!(format!("path_extension: \"{}\" has no extension", s))),
        },
        other => verr!(vs!(format!(
            "path_extension: expects a string, got {}",
            other.type_name()
        ))),
    }
}
