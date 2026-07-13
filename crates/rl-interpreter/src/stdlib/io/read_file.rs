use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn std_read_file(_: &mut Evaluator, file: String) -> Value {
    let data = match std::fs::read_to_string(&file) {
        Ok(d) => d,
        Err(e) => {
            return verr!(vs!(format!(
                "read_file: failed to read \"{}\": {}",
                file, e
            )));
        }
    };
    vok!(vs!(data))
}
