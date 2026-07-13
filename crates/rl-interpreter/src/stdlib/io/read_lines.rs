use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_read_lines(_: &mut Evaluator, file: String) -> Value {
    let data = match std::fs::read_to_string(&file) {
        Err(e) => {
            return verr!(vs!(format!(
                "read_lines: failed to read \"{}\": {}",
                file, e
            )));
        }
        Ok(d) => d
            .lines()
            .map(String::from)
            .map(Value::String)
            .collect::<Vec<Value>>(),
    };

    vok!(Value::Values {
        items_type: TypeAnnotation::String,
        items: data,
    })
}
