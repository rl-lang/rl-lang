use crate::interpreter::{evaluator::Evaluator, values::Value};

pub fn func(eval: &mut Evaluator, value: Value) -> Value {
    let text = format!("[dbg] {} ({})\n", value, value.type_name());

    if let Some(buffer) = &mut eval.output_buffer {
        buffer.push_str(&text);
    } else {
        eprint!("{}", text);
    }

    value
}
