use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_rmdir(_: &mut Evaluator, path: String) -> Value {
    if let Err(e) = std::fs::remove_dir(&path) {
        return verr!(vs!(format!("rmdir: failed to delete \"{}\": {}", path, e)));
    };
    vok!(vnl!())
}
