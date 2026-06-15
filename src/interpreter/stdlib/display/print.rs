use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;

pub fn std_print(evaluator: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    let text = args.iter().map(|s| s.to_string()).collect::<String>();

    if let Some(buffer) = &mut evaluator.output_buffer {
        buffer.push_str(&text);
    } else {
        print!("{}", text);
    }
    Ok(Value::Null)
}
