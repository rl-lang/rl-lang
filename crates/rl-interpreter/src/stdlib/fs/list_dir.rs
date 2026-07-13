use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};
use rl_ast::statements::TypeAnnotation;

pub fn std_list_dir(_: &mut Evaluator, path: String) -> Value {
    match std::fs::read_dir(&path) {
        Err(e) => {
            verr!(vs!(format!("list_dir: failed to read \"{}\": {}", path, e)))
        }
        Ok(d) => {
            vok!(Value::Values {
                items_type: TypeAnnotation::String,
                items: d
                    .filter_map(|i| i.ok())
                    .map(|i| i.path().to_string_lossy().to_string())
                    .map(Value::String)
                    .collect::<Vec<Value>>()
            })
        }
    }
}
