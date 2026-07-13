use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_pop(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => {
            let mut buf = std::path::PathBuf::from(&s);
            buf.pop();
            vok!(vs!(buf.to_string_lossy().to_string()))
        }
        other => verr!(vs!(format!(
            "path_pop: expects a string, got {}",
            other.type_name()
        ))),
    }
}
