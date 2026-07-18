use crate::evaluator::Evaluator;

pub fn std_path_join(_: &mut Evaluator, path: String, target: String) -> String {
    std::path::PathBuf::from(&path)
        .join(&target)
        .to_string_lossy()
        .to_string()
}
