use crate::evaluator::Evaluator;

pub fn std_path_push(_: &mut Evaluator, path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.push(&target);
    buf.to_string_lossy().to_string()
}
