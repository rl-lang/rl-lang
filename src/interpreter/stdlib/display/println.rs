use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;

pub fn std_println(evaluator: &mut Evaluator, args: Vec<Value>) -> Result<Value, Error> {
    let text = args
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i > 0 {
                format!(" {}", a)
            } else {
                format!("{}", a)
            }
        })
        .collect::<String>();

    if let Some(buffer) = &mut evaluator.output_buffer {
        buffer.push_str(&text);
        buffer.push('\n');
    } else {
        println!("{}", text);
    }
    Ok(Value::Null)
}
