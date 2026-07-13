use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_cwd(_: &mut Evaluator) -> Value {
    match std::env::current_dir() {
        Ok(p) => vok!(vs!(p.to_string_lossy().to_string())),
        Err(e) => verr!(vs!(format!("cwd: {}", e))),
    }
}

pub fn std_set_cwd(_: &mut Evaluator, path: String) -> Value {
    match std::env::set_current_dir(&path) {
        Ok(_) => vok!(vnl!()),
        Err(e) => {
            verr!(vs!(format!(
                "set_cwd: failed to change to \"{}\": {}",
                path, e
            )))
        }
    }
}
