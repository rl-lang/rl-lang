use crate::{evaluator::Evaluator, values::Value};
use rl_utils::{errors::Error, span::Span};

pub fn std_print(eval: &mut Evaluator, args: Vec<Value>, _: Span) -> Result<Value, Error> {
    let text = args.iter().map(|s| s.to_string()).collect::<String>();

    if let Some(buffer) = &mut eval.output_buffer {
        buffer.push_str(&text);
    } else {
        print!("{}", text);
    }
    Ok(Value::Null)
}
