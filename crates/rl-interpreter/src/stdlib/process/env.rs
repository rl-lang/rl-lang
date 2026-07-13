use crate::{
    evaluator::Evaluator,
    stdlib::common::{vnl, vs},
    values::Value,
};

pub fn std_env(_: &mut Evaluator, key: String) -> Value {
    match std::env::var(&key) {
        Ok(val) => vs!(val),
        Err(_) => vnl!(),
    }
}
