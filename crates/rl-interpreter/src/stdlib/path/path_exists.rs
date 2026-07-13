use crate::{
    evaluator::Evaluator,
    stdlib::common::{vb, verr, vok, vs},
    values::Value,
};

pub fn std_path_exists(_: &mut Evaluator, path: Value) -> Value {
    match path {
        Value::String(s) => vok!(vb!(std::path::Path::new(&s).exists())),
        other => verr!(vs!(format!(
            "path_exists expects a string, got {}",
            other.type_name()
        ))),
    }
}
