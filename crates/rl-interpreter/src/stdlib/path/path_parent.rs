use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_path_parent(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => match std::path::Path::new(&s).parent() {
            Some(p) => vok!(vs!(p.to_string_lossy().to_string())),
            None => verr!(vs!(format!("path_parent: \"{}\" has no parent", s))),
        },
        other => verr!(vs!(format!(
            "path_parent: expects a string, got {}",
            other.type_name()
        ))),
    }
}
