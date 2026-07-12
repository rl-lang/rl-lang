use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_mkdir_all(_: &mut Evaluator, path: String) -> Value {
    if let Err(e) = std::fs::create_dir_all(&path) {
        return verr!(vs!(format!(
            "mkdir_all: failed to create \"{}\": {}",
            path, e
        )));
    };
    vok!(vnl!())
}
