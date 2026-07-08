use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vi, vok, vs},
    values::Value,
};

pub fn std_copy_file(_: &mut Evaluator, src: String, dst: String) -> Value {
    let bytes = match std::fs::copy(&src, &dst) {
        Ok(b) => b,
        Err(e) => {
            return verr!(vs!(format!(
                "copy_file(): failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            )));
        }
    };
    vok!(vi!(bytes as i64))
}
