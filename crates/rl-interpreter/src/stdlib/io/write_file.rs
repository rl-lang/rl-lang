use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_write_file(_: &mut Evaluator, file: String, content: String) -> Value {
    match std::fs::write(&file, content) {
        Ok(_) => vok!(vnl!()),
        Err(e) => {
            return verr!(vs!(format!(
                "write_file: failed to write \"{}\": {}",
                file, e
            )));
        }
    }
}
