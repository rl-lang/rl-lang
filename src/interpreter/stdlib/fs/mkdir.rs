use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_mkdir(_: &mut Evaluator, path: String) -> Value {
    if let Err(e) = std::fs::create_dir(&path) {
        return verr!(vs!(format!("mkdir: failed to create \"{}\": {}", path, e)));
    };
    vok!(vnl!())
}
