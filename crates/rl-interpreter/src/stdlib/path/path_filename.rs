use crate::{
    evaluator::Evaluator,
    stdlib::common::{vnl, vs},
    values::Value,
};

pub fn std_path_filename(_: &mut Evaluator, path: String) -> Value {
    match std::path::Path::new(&path).file_name() {
        Some(name) => vs!(name.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
