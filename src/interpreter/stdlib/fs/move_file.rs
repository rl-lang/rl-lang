use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::common::{verr, vnl, vok, vs},
    values::Value,
};

pub fn std_move_file(_: &mut Evaluator, src: String, dst: String) -> Value {
    match std::fs::rename(&src, &dst) {
        Err(e) => {
            return verr!(vs!(format!(
                "move_file: failed to move \"{}\" to \"{}\": {}",
                src, dst, e
            )));
        }
        Ok(_) => {}
    };
    vok!(vnl!())
}
