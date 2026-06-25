use crate::{
    interpreter::{evaluator::Evaluator, values::Value},
    utils::{errors::Error, span::Span},
};

pub fn std_to_bool(eval: &mut Evaluator, value: Value, span: Span) -> Result<bool, Error> {
    match value {
        Value::Bool(b) => Ok(b),
        Value::Integer(i) => Ok(i != 0),
        Value::Byte(i) => Ok(i != 0),
        Value::Float(f) => Ok(f != 0.0),
        Value::Null => Ok(false),
        Value::String(s) => match s.trim() {
            "true" | "1" => Ok(true),
            "false" | "0" | "" => Ok(false),
            _ => Ok(true),
        },

        other => Err(eval.err(
            format!("cannot parse \"{}\" as bool", other.type_name()),
            span,
        )),
    }
}
