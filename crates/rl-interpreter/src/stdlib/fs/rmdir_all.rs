use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_rmdir_all(_: &mut Evaluator, path: String) -> Value {
    if let Err(e) = std::fs::remove_dir_all(&path) {
        return verr!(vs!(format!(
            "rmdir_all: failed to delete \"{}\": {}",
            path, e
        )));
    };
    vok!(vnl!())
}
