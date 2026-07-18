use crate::{
    evaluator::Evaluator,
    stdlib::common::{vnl, vs},
    values::Value,
};

pub fn std_path_extension(_: &mut Evaluator, path: String) -> Value {
    match std::path::Path::new(&path).extension() {
        Some(ext) => vs!(ext.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
