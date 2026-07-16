use crate::evaluator::Evaluator;

pub fn std_path_is_file(_: &mut Evaluator, path: String) -> bool {
    std::path::Path::new(&path).is_file()
}
