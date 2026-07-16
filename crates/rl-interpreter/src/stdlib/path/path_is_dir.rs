use crate::evaluator::Evaluator;

pub fn std_path_is_dir(_: &mut Evaluator, path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}
