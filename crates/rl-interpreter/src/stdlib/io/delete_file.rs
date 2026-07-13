use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_delete_file(_: &mut Evaluator, file: String) -> Value {
    match std::fs::remove_file(&file) {
        Ok(_) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!(
            "delete_file: failed to read \"{}\": {}",
            file, e
        ))),
    }
}
