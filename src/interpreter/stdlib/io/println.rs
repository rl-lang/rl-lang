use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::values::Value;
use crate::utils::errors::Error;
use crate::utils::span::Span;

pub fn std_println(evaluator: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    let text = args.iter().map(|s| s.to_string()).collect::<String>();

    if let Some(buffer) = &mut evaluator.output_buffer {
        buffer.push_str(&text);
        buffer.push('\n');
    } else {
        println!("{}", text);
    }
    Ok(Value::Null)
}
