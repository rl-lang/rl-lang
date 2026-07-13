use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_is_dir(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => vok!(Value::Bool(std::path::Path::new(&s).is_dir())),
        other => verr!(vs!(format!(
            "path_is_dir: expects a string, got {}",
            other.type_name()
        ))),
    }
}
