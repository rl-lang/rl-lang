use crate::{
    evaluator::Evaluator,
    stdlib::common::{vnl, vs},
    values::Value,
};

pub fn std_path_parent(_: &mut Evaluator, path: String) -> Value {
    match std::path::Path::new(&path).parent() {
        Some(p) => vs!(p.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
