use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn std_concat(_: &mut Evaluator, args: Vec<Value>) -> String {
    args.iter().map(|a| format!("{}", a)).collect::<String>()
}
