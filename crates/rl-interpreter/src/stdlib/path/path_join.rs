use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_join(_: &mut Evaluator, path: Value, target: String) -> Value {
    match path {
        Value::String(s) => vok!(vs!(std::path::PathBuf::from(&s)
            .join(&target)
            .to_string_lossy()
            .to_string())),
        other => verr!(vs!(format!(
            "path_join: expects a string, got {}",
            other.type_name()
        ))),
    }
}
