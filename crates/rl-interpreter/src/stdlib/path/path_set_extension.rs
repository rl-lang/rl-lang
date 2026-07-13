use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_set_extension(_: &mut Evaluator, path: Value, target: String) -> Value {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.set_extension(&target);
            vok!(vs!(buf.to_string_lossy().to_string()))
        }
        other => verr!(vs!(format!(
            "path_set_extension: expects a string, got {}",
            other.type_name()
        ))),
    }
}
