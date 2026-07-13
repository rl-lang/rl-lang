use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn func(_: &mut Evaluator, file: String) -> Value {
    let data = match std::fs::read(&file) {
        Err(e) => {
            return verr!(vs!(format!(
                "read_bytes: failed to read \"{}\": {}",
                file, e
            )));
        }
        Ok(d) => d.into_iter().map(Value::Byte).collect::<Vec<Value>>(),
    };

    vok!(Value::Values {
        items_type: TypeAnnotation::Byte,
        items: data,
    })
}
