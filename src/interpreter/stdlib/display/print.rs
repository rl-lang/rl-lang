use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;

pub fn std_print(evaluator: &mut Evaluator, args: Vec<Value>) -> Value {
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
    } else {
        print!("{}", text);
    }
    Value::Null
}
