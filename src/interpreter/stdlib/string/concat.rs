use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::errors::Error,
};

pub fn std_concat(_: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::String(
        args.iter().map(|a| format!("{}", a)).collect::<String>(),
    ))
}
