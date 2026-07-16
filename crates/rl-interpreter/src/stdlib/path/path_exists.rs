use crate::evaluator::Evaluator;

pub fn std_path_exists(_: &mut Evaluator, path: String) -> bool {
    std::path::Path::new(&path).exists()
}
