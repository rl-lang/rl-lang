use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_move_file(_: &mut Evaluator, src: String, dst: String) -> Value {
    if let Err(e) = std::fs::rename(&src, &dst) {
        return verr!(vs!(format!(
            "move_file: failed to move \"{}\" to \"{}\": {}",
            src, dst, e
        )));
    };
    vok!(vnl!())
}
