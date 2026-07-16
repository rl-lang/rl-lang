use crate::{
    evaluator::Evaluator,
    stdlib::common::{vnl, vs},
    values::Value,
};

pub fn std_path_stem(_: &mut Evaluator, path: String) -> Value {
    match std::path::Path::new(&path).file_stem() {
        Some(stem) => vs!(stem.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
