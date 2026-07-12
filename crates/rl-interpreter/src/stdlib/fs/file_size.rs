use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vi, vok, vs},
    values::Value,
};

pub fn std_file_size(_: &mut Evaluator, path: String) -> Value {
    match std::fs::metadata(&path) {
        Err(e) => verr!(vs!(format!(
            "file_size: failed to read \"{}\": {}",
            path, e
        ))),

        Ok(metadata) => vok!(vi!(metadata.len() as i64)),
    }
}
